use anyhow::Result;
use sqlx::SqlitePool;
use tracing::instrument;
use tracing_futures::Instrument;

use crate::{
    documents::{document_gatekeeper::DocumentGatekeeper, document_service::DocumentService},
    watches::{file_watcher::FileWatcherEventReceiver, watch_repository},
};

pub struct WatchEventHandler {
    event_rx: FileWatcherEventReceiver,
    document_service: DocumentService,
    connection_pool: SqlitePool,
}

impl WatchEventHandler {
    pub fn new(
        event_rx: FileWatcherEventReceiver,
        document_service: DocumentService,
        connection_pool: SqlitePool,
    ) -> Self {
        Self {
            event_rx,
            document_service,
            connection_pool,
        }
    }

    #[instrument(name = "WatchEventHandler::run", level = "info", skip(self))]
    pub async fn run(mut self) -> Result<()> {
        use futures::StreamExt;

        use crate::watches::FileWatcherEvent::*;

        while let Ok(event) = self
            .event_rx
            .recv()
            .instrument(tracing::trace_span!("event loop"))
            .await
        {
            tracing::trace!("Received event: {:?}", event);
            let path = event.get_first_path();

            match &event {
                Created(_) => {
                    let mut conn = self.connection_pool.acquire().await?;
                    match self.document_service.add_document(path, &mut conn).await {
                        Ok(_) => {
                            self.document_service.commit_to_search_engine().await?;
                        }
                        Err(e) => {
                            use crate::documents::document_service::AddDocumentError::*;
                            match e {
                                NotEligible => { /* ignore */ }
                                AlreadyExists => { /* ignore */ }
                                NoWatchFound => { /* ignore */ }
                                Other(e) => {
                                    tracing::error!("Error adding document: {}", e);
                                }
                            }
                        }
                    }
                }
                Modified(_) => {
                    let mut conn = self.connection_pool.acquire().await?;
                    match self.document_service.update_document(path, &mut *conn).await {
                        Ok(_) => {
                            self.document_service.commit_to_search_engine().await?;
                        }
                        Err(e) => {
                            use crate::documents::document_service::UpdateDocumentError::*;
                            match e {
                                NotEligible => { /* ignore */ }
                                NotExists => {
                                    tracing::warn!("Document {} does not exist.", path.display());
                                }
                                NoWatchFound => { /* ignore */ }
                                Other(e) => {
                                    tracing::error!("Error updating document: {}", e);
                                }
                            }
                        }
                    }
                }
                Removed(_) => {
                    match self.document_service.delete_document(path).await {
                        Ok(()) => {
                            self.document_service.commit_to_search_engine().await?;
                        }
                        Err(e) => {
                            use crate::documents::document_service::DeleteDocumentError::*;
                            match e {
                                NotExists => { /* ignore */ }
                                Other(e) => {
                                    tracing::error!("Error deleting document: {}", e);
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
