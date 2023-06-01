use std::{path::PathBuf, time::Instant};

use anyhow::Result;
use futures::channel::mpsc;
use sqlx::SqlitePool;
use tokio::task::{join_set, JoinError, JoinHandle};
use tracing::{instrument, Instrument};

use super::tasks::{get_documents::get_documents, update_documents::update_documents};
use crate::{
    documents::{document_gatekeeper::DocumentGatekeeper, DocumentService},
    watches::{
        jobs::{
            parallelism::available_parallelism,
            sync_watch_job::report::report,
            tasks::{
                add_documents::{add_documents, AddDocumentsEvent},
                scan_directory::{scan_directory, ScanDirectoryEvent},
            },
            JobId,
        },
        watch_repository, WatchEvent, WatchStatus,
    },
    Watch,
};

mod report;

/// ファイルシステムとデータベース、インデックスを同期するジョブ
///
/// - インデックスに登録済みのファイルがファイルシステムに存在しない場合は、インデックスから削除する
/// - インデックスに登録済みのファイルがファイルシステム上で更新されている場合は、インデックスを更新する
/// - インデックスに未登録のファイルがファイルシステムに存在する場合は、インデックスに追加する
#[derive(Clone)]
pub struct SyncWatchJob {
    document_service: DocumentService,
    document_gatekeeper: DocumentGatekeeper,
    connection_pool: SqlitePool,
    watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
}

impl SyncWatchJob {
    pub fn new(
        document_service: DocumentService,
        document_gatekeeper: DocumentGatekeeper,
        connection_pool: SqlitePool,
        watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
    ) -> Self {
        Self {
            document_service,
            document_gatekeeper,
            connection_pool,
            watch_event_tx,
        }
    }

    #[instrument(name = "SyncWatchJob::start", level = "info", skip(self))]
    pub async fn start(mut self, job_id: JobId, watch: Watch) -> Result<()> {
        let start = Instant::now();
        let parallelism = available_parallelism();
        let mut join_set = tokio::task::JoinSet::new();

        let (document_tx, document_rx) = async_channel::unbounded();
        let (path_tx, path_rx) = async_channel::bounded::<PathBuf>(10_000);

        let (get_documents_event_tx, get_documents_event_rx) = mpsc::unbounded();
        let (update_documents_event_tx, update_documents_event_rx) = mpsc::unbounded();
        let (scan_directory_event_tx, scan_directory_event_rx) = mpsc::channel::<ScanDirectoryEvent>(0);
        let (add_documents_event_tx, add_documents_event_rx) = mpsc::channel::<AddDocumentsEvent>(0);

        join_set.spawn(report(
            watch.clone(),
            get_documents_event_rx,
            update_documents_event_rx,
            add_documents_event_rx,
            scan_directory_event_rx,
            self.watch_event_tx.clone(),
        ));

        // get_documents は documents テーブルに長時間アクセスするため書き込み処理に影響がある (database is locked) ので先に終わらせる。
        // チャネルの容量は無制限にしておく
        join_set.spawn(get_documents(
            watch.id,
            document_tx,
            get_documents_event_tx,
            self.document_service.clone(),
            self.connection_pool.clone(),
        ));
        if let Some(result) = join_set.join_next().await {
            flatten(result)?;
        }

        for _ in 0..usize::max(1, parallelism / 2) {
            join_set.spawn(update_documents(
                document_rx.clone(),
                update_documents_event_tx.clone(),
                self.document_service.clone(),
                self.document_gatekeeper.clone(),
            ));
        }
        drop(update_documents_event_tx);

        join_set.spawn(scan_directory(
            watch.path.clone(),
            self.document_gatekeeper.clone(),
            path_tx,
            scan_directory_event_tx,
        ));

        for _ in 0..usize::max(1, parallelism / 2) {
            join_set.spawn(
                add_documents(
                    watch.id,
                    path_rx.clone(),
                    add_documents_event_tx.clone(),
                    self.document_service.clone(),
                )
                .instrument(tracing::info_span!("SyncWatchJob", job_id = %job_id)),
            );
        }
        drop(add_documents_event_tx);

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    tracing::error!("error occurred while running SyncWatchJob ({}): {}", job_id, e);
                }
                Err(e) => {
                    tracing::error!("{}", e);
                }
            }
        }

        self.document_service.commit_to_search_engine().await?;

        let mut conn = self.connection_pool.acquire().await?;
        watch_repository::update(watch.id, WatchStatus::Active, &mut *conn).await?;

        tracing::info!(
            "Finished SyncWatchJob for watch {watch_id} in {elapsed} seconds",
            watch_id = watch.id,
            elapsed = start.elapsed().as_secs()
        );

        Ok(())
    }
}

fn flatten<T>(result: Result<Result<T>, JoinError>) -> Result<T> {
    match result {
        Ok(Ok(t)) => Ok(t),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(e.into()),
    }
}
