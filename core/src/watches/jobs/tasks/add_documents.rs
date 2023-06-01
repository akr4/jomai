use std::{ops::Deref, path::PathBuf};

use anyhow::Result;
use futures::{channel::mpsc, SinkExt, StreamExt};
use sqlx::{Acquire, SqlitePool};
use tracing::instrument;
use tracing_futures::Instrument;

use crate::{documents, watches::jobs::JobId, WatchId};

#[derive(Debug)]
pub enum AddDocumentsEvent {
    DocumentAdded,
    AlreadyExists,
    AddingDocumentFailed,
}

pub type AddDocumentsEventSender = mpsc::Sender<AddDocumentsEvent>;
pub type AddDocumentsEventReceiver = mpsc::Receiver<AddDocumentsEvent>;

#[instrument(skip(path_rx, event_tx, document_service))]
pub async fn add_documents(
    watch_id: WatchId,
    mut path_rx: async_channel::Receiver<PathBuf>,
    mut event_tx: AddDocumentsEventSender,
    mut document_service: documents::DocumentService,
) -> Result<()> {
    while let Some(path) = path_rx.next().instrument(tracing::trace_span!("path_rx loop")).await {
        // 高負荷時に書き込みが競合して database is locked エラーが発生する。
        // https://gist.github.com/akr4/a9eb6ed6d7bae941941c43e236eaef60
        // 回避のためにトランザクションを使わない。
        match document_service.add_document_with_watch(&path, watch_id).await {
            Ok(()) => {
                event_tx.send(AddDocumentsEvent::DocumentAdded).await?;
            }
            Err(e) => {
                use crate::documents::document_service::AddDocumentError::*;
                match e {
                    AlreadyExists => {
                        event_tx.send(AddDocumentsEvent::AlreadyExists).await?;
                    }
                    NotEligible | NoWatchFound => {
                        event_tx.send(AddDocumentsEvent::AddingDocumentFailed).await?;
                    }
                    Other(e) => {
                        tracing::warn!("error occurred while registering the document: {}", e);
                        event_tx.send(AddDocumentsEvent::AddingDocumentFailed).await?;
                    }
                }
            }
        }
    }

    Ok(())
}
