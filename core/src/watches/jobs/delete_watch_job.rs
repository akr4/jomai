use anyhow::Result;
use futures::{channel::mpsc, SinkExt};
use sqlx::SqlitePool;
use tracing::instrument;

use crate::{
    documents::DocumentService,
    watches,
    watches::{watch_repository, WatchEvent},
    Watch,
};

/// データベースの watch 情報をもとにインデックスから削除するジョブ
#[derive(Clone)]
pub struct DeleteWatchJob {
    connection_pool: SqlitePool,
    event_tx: mpsc::UnboundedSender<WatchEvent>,
    document_service: DocumentService,
}

impl DeleteWatchJob {
    pub fn new(
        connection_pool: SqlitePool,
        event_tx: mpsc::UnboundedSender<WatchEvent>,
        document_service: DocumentService,
    ) -> Self {
        Self {
            connection_pool,
            event_tx,
            document_service,
        }
    }

    #[instrument(name = "DeleteWatchJob::start", level = "info", skip(self))]
    pub async fn start(mut self, watch: Watch) -> Result<()> {
        self.event_tx
            .send(WatchEvent::DeleteWatchStarted { watch: watch.clone() })
            .await?;
        let mut tx = self.connection_pool.begin().await?;

        self.document_service.delete_documents_by_watch_id(watch.id).await?;
        self.document_service.commit_to_search_engine().await?;

        watch_repository::delete(watch.id, &mut *tx).await?;
        tx.commit().await?;

        self.event_tx
            .send(watches::WatchEvent::DeleteWatchFinished { watch: watch.clone() })
            .await?;

        tracing::info!("Finished DeleteWatchJob for watch {}", watch.id);
        Ok(())
    }
}
