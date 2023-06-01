use std::{future::Future, path::PathBuf, process::Output, time::Instant};

use anyhow::Result;
use futures::{channel::mpsc, future::BoxFuture, stream::FuturesUnordered, FutureExt, SinkExt, StreamExt};
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::task::{AbortHandle, JoinHandle};
use tracing::{instrument, Instrument};

use crate::{
    documents,
    documents::document_gatekeeper::DocumentGatekeeper,
    watches::{
        jobs::{parallelism::available_parallelism, JobId},
        watch_repository, WatchEvent, WatchStatus,
    },
    Watch,
};

use super::tasks::{
    add_documents::{add_documents, AddDocumentsEvent},
    scan_directory::{scan_directory, ScanDirectoryEvent},
};

use self::report::report;

mod report;

#[derive(Error, Debug)]
pub enum ScanWatchJobError {
    #[error("the document should be excluded")]
    ShouldExclude,
    #[error("the document is not eligible")]
    NotEligible,
    #[error("the document already exists")]
    AlreadyExists,
    #[error("error occurred while adding the document")]
    Other(#[from] anyhow::Error),
}

/// ファイルシステムをスキャンしてインデックス未登録のファイルを登録するジョブ
#[derive(Clone)]
pub struct ScanWatchJob {
    connection_pool: SqlitePool,
    document_service: documents::DocumentService,
    document_gatekeeper: DocumentGatekeeper,
    watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
}

impl ScanWatchJob {
    pub fn new(
        connection_pool: SqlitePool,
        document_service: documents::DocumentService,
        document_gatekeeper: DocumentGatekeeper,
        watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
    ) -> Self {
        ScanWatchJob {
            connection_pool,
            document_service,
            document_gatekeeper,
            watch_event_tx,
        }
    }

    #[instrument(name = "ScanWatchJob::start", level = "info", skip(self))]
    pub async fn start(mut self, job_id: JobId, watch: Watch) -> Result<()> {
        let start = Instant::now();
        let parallelism = available_parallelism();
        let mut join_set = tokio::task::JoinSet::new();

        self.watch_event_tx
            .send(WatchEvent::AddWatchStarted { watch: watch.clone() })
            .await?;

        let (path_tx, path_rx) = async_channel::unbounded::<PathBuf>();
        let (scan_directory_event_tx, scan_directory_event_rx) = mpsc::channel::<ScanDirectoryEvent>(0);
        let (add_documents_event_tx, add_documents_event_rx) = mpsc::channel::<AddDocumentsEvent>(0);

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
                .instrument(tracing::info_span!("ScanWatchJob", job_id = %job_id)),
            );
        }
        drop(add_documents_event_tx);

        join_set.spawn(report(
            watch.clone(),
            add_documents_event_rx,
            scan_directory_event_rx,
            self.watch_event_tx.clone(),
        ));

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    tracing::error!("error occurred while running SycWatchJob ({}): {}", job_id, e);
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
            "Finished ScanWatchJob for watch {watch_id} in {elapsed} seconds",
            watch_id = watch.id,
            elapsed = start.elapsed().as_secs()
        );
        Ok(())
    }
}
