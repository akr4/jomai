use anyhow::Result;
use futures::{channel::mpsc, SinkExt, StreamExt};
use sqlx::SqlitePool;
use tracing::instrument;
use tracing_futures::Instrument;

use crate::{
    documents::{Document, DocumentService},
    WatchId,
};

#[derive(Debug)]
pub enum GetDocumentsEvent {
    DocumentFound,
}

#[instrument(skip(document_tx, event_tx, document_service, connection_pool))]
pub async fn get_documents(
    watch_id: WatchId,
    document_tx: async_channel::Sender<Document>,
    mut event_tx: mpsc::UnboundedSender<GetDocumentsEvent>,
    document_service: DocumentService,
    connection_pool: SqlitePool,
) -> Result<()> {
    let mut conn = connection_pool.acquire().await?;
    let mut stream = document_service.find_documents_by_watch_id(watch_id, &mut *conn)?;
    while let Some(result) = stream.next().instrument(tracing::trace_span!("stream loop")).await {
        match result {
            Ok(document) => {
                document_tx.send(document).await?;
                event_tx.send(GetDocumentsEvent::DocumentFound).await?;
            }
            Err(e) => {
                tracing::error!("{}", e);
            }
        }
    }

    Ok(())
}
