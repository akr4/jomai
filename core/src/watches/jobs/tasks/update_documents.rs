use anyhow::Result;
use chrono::Utc;
use futures::{channel::mpsc, SinkExt, StreamExt};
use sqlx::SqlitePool;
use tracing::{instrument, Instrument};

use crate::documents::document_gatekeeper::DocumentGatekeeper;
use crate::documents::{Document, DocumentService};

#[derive(Debug)]
pub enum UpdateDocumentsEvent {
    UpToDate,
    DocumentDeleted,
    DeletingDocumentFailed,
    DocumentUpdated,
    UpdatingDocumentFailed,
}

#[instrument(skip(document_rx, event_tx, document_service, document_gatekeeper))]
pub async fn update_documents(
    mut document_rx: async_channel::Receiver<Document>,
    mut event_tx: mpsc::UnboundedSender<UpdateDocumentsEvent>,
    mut document_service: DocumentService,
    document_gatekeeper: DocumentGatekeeper,
) -> Result<()> {
    while let Some(document) = document_rx
        .next()
        .instrument(tracing::trace_span!("document_rx loop"))
        .await
    {
        let path = document.path;
        if !document_gatekeeper.is_eligible(&path) {
            match document_service.delete_document(&path).await {
                Ok(_) => {
                    event_tx.send(UpdateDocumentsEvent::DocumentDeleted).await?;
                }
                Err(e) => {
                    tracing::error!("Failed to delete document {}, {:?}", path.display(), e);
                    event_tx.send(UpdateDocumentsEvent::DeletingDocumentFailed).await?;
                }
            }
        } else {
            let file_modified_at = chrono::DateTime::<Utc>::from(path.metadata()?.modified()?);
            if document.indexed_at.is_none() || document.indexed_at.unwrap() < file_modified_at {
                match document_service
                    .update_document_with_watch_id(&path, document.watch_id)
                    .await
                {
                    Ok(_) => {
                        event_tx.send(UpdateDocumentsEvent::DocumentUpdated).await?;
                    }
                    Err(e) => {
                        tracing::error!("Failed to update document {}, {:?}", path.display(), e);
                        event_tx.send(UpdateDocumentsEvent::UpdatingDocumentFailed).await?;
                    }
                }
            } else {
                event_tx.send(UpdateDocumentsEvent::UpToDate).await?;
            }
        }
    }

    Ok(())
}
