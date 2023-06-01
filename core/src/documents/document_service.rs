use std::{fmt::Debug, path::Path};

use anyhow::{anyhow, Result};
use futures::{
    channel::{mpsc, mpsc::SendError},
    SinkExt,
};
use sqlx::{pool::PoolConnection, Sqlite, SqliteConnection, Transaction};
use tantivy::{doc, IndexWriter};
use thiserror::Error;
use tokio::sync::oneshot;

use crate::{
    documents::{
        document_gatekeeper::DocumentGatekeeper,
        document_repository,
        document_repository::RepositoryError,
        search::{Search, SearchResults, Sort},
        Document, IndexWriterCommand,
    },
    watches::watch_repository,
    WatchId,
};

#[derive(Debug, Clone)]
pub enum DocumentEvent {
    DocumentAdded(Document),
    DocumentUpdated(Document),
    /// a document was deleted. note that there is no event for multiple documents being deleted
    DocumentDeleted(Document),
}

#[derive(Clone)]
pub struct DocumentService {
    search: Search,
    command_tx: mpsc::Sender<document_repository::document_db_command_processor::Command>,
    index_writer_command_tx: mpsc::Sender<IndexWriterCommand>,
    document_event_tx: tokio::sync::broadcast::Sender<DocumentEvent>,
}

#[derive(Error, Debug)]
pub enum AddDocumentError {
    #[error("the document is not eligible")]
    NotEligible,
    #[error("no corresponding watch found")]
    NoWatchFound,
    #[error("the document already exists")]
    AlreadyExists,
    #[error("error occurred while adding the document")]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum UpdateDocumentError {
    #[error("the document is not eligible")]
    NotEligible,
    #[error("no corresponding watch found")]
    NoWatchFound,
    #[error("the document does not exist")]
    NotExists,
    #[error("error occurred while updating the document")]
    Other(#[from] anyhow::Error),
}

#[derive(Error, Debug)]
pub enum DeleteDocumentError {
    #[error("the document does not exist")]
    NotExists,
    #[error("error occurred while updating the document")]
    Other(#[from] anyhow::Error),
}

impl DocumentService {
    pub fn new(
        command_tx: mpsc::Sender<document_repository::document_db_command_processor::Command>,
        index_writer_command_tx: mpsc::Sender<IndexWriterCommand>,
        search: Search,
    ) -> Result<Self> {
        let (document_event_tx, _) = tokio::sync::broadcast::channel(100);
        Ok(Self {
            search,
            index_writer_command_tx,
            command_tx,
            document_event_tx,
        })
    }

    pub async fn add_document_with_watch<P: AsRef<Path>>(
        &mut self,
        path: P,
        watch_id: WatchId,
    ) -> Result<(), AddDocumentError> {
        let path = path.as_ref();

        let document = match document_repository::insert(path, watch_id, None, self.command_tx.clone()).await {
            Ok(document) => document,
            Err(e) => {
                return match e {
                    RepositoryError::UniqueConstraintViolation => Err(AddDocumentError::AlreadyExists),
                    _ => Err(AddDocumentError::Other(e.into())),
                };
            }
        };

        tracing::trace!("Registering document: {}", path.display());
        match self.add_document_to_search_engine(path, watch_id).await {
            Ok(()) => {
                tracing::debug!("Registered document: {}", path.display());
                self.document_event_tx
                    .send(DocumentEvent::DocumentAdded(document))
                    .map_err(|e| anyhow!(e))?;
            }
            Err(e) => {
                tracing::error!(
                    "Failed to register document: {}, cleaning up the inserted data on the db.",
                    e
                );
                document_repository::delete_by_path(path, self.command_tx.clone())
                    .await
                    .map_err(|e| AddDocumentError::Other(e.into()))?;
                return Err(AddDocumentError::Other(e.into()));
            }
        }

        Ok(())
    }

    pub async fn add_document<P: AsRef<Path>>(
        &mut self,
        path: P,
        conn: &mut SqliteConnection,
    ) -> Result<(), AddDocumentError> {
        let path = path.as_ref();

        return match watch_repository::find_containing_path(&path, conn).await? {
            None => {
                tracing::warn!("path ({}) is not managed by any watch", path.display());
                Err(AddDocumentError::NoWatchFound)
            }
            Some(watch) => self.add_document_with_watch(path, watch.id).await,
        };
    }

    pub async fn delete_document<P: AsRef<Path>>(&mut self, path: P) -> Result<(), DeleteDocumentError> {
        let path = path.as_ref();
        let document = document_repository::delete_by_path(path, self.command_tx.clone())
            .await
            .map_err(|e| anyhow!(e))?;
        if document.is_none() {
            return Err(DeleteDocumentError::NotExists);
        }
        let document = document.unwrap();
        self.delete_document_from_search_engine(path).await?;
        self.document_event_tx
            .send(DocumentEvent::DocumentDeleted(document))
            .map_err(|e| anyhow!(e))?;
        Ok(())
    }

    pub async fn delete_documents_by_watch_id(&mut self, watch_id: WatchId) -> Result<()> {
        document_repository::delete_by_watch_id(watch_id, self.command_tx.clone()).await?;
        let (result_tx, result_rx) = oneshot::channel();
        self.index_writer_command_tx
            .send(IndexWriterCommand::DeleteByWatchId { watch_id, result_tx })
            .await?;
        result_rx.await.map_err(|e| anyhow!(e))?
    }

    pub async fn update_document_with_watch_id<P: AsRef<Path>>(
        &mut self,
        path: P,
        watch_id: WatchId,
    ) -> Result<(), UpdateDocumentError> {
        let path = path.as_ref();

        self.delete_document_from_search_engine(path).await?;
        self.add_document_to_search_engine(path, watch_id).await?;
        let document = document_repository::update_by_path(path, chrono::Utc::now(), self.command_tx.clone())
            .await
            .map_err(|e| anyhow!(e))?;
        if document.is_none() {
            return Err(UpdateDocumentError::NotExists);
        }
        let document = document.unwrap();
        self.document_event_tx
            .send(DocumentEvent::DocumentUpdated(document))
            .map_err(|e| anyhow!(e))?;

        Ok(())
    }

    pub async fn update_document<P: AsRef<Path>>(
        &mut self,
        path: P,
        conn: &mut SqliteConnection,
    ) -> Result<(), UpdateDocumentError> {
        let path = path.as_ref();

        match watch_repository::find_containing_path(&path, &mut *conn).await? {
            None => {
                tracing::warn!("Document ({}) is not registered in any watch", path.display());
                return Err(UpdateDocumentError::NoWatchFound);
            }
            Some(watch) => {
                self.update_document_with_watch_id(path, watch.id).await?;
            }
        }

        Ok(())
    }

    pub fn get_all_documents(&self, offset: usize, limit: usize) -> Result<SearchResults> {
        self.search.get_all_documents(offset, limit)
    }

    pub async fn find_document_by_path<P: AsRef<Path> + Debug>(&self, path: P) -> Result<Option<Document>> {
        document_repository::find_by_path(path, self.command_tx.clone())
            .await
            .map_err(|e| anyhow!(e))
    }

    pub fn find_documents_by_watch_id<'a>(
        &self,
        watch_id: WatchId,
        conn: &'a mut SqliteConnection,
    ) -> Result<impl futures::Stream<Item = Result<Document>> + Send + 'a> {
        document_repository::find_by_watch_id(watch_id, conn).map_err(|e| anyhow!(e))
    }

    pub fn search_documents(
        &self,
        query: &str,
        tags: &[&str],
        sort: Sort,
        offset: usize,
        limit: usize,
    ) -> Result<SearchResults> {
        if tags.is_empty() {
            self.search.search_document(query, sort, offset, limit)
        } else {
            self.search.search_document_with_tags(query, tags, sort, offset, limit)
        }
    }

    pub fn count_documents_under_path<P: AsRef<Path>>(&self, path: P) -> Result<u32> {
        self.search.count_documents_under_path(path)
    }

    async fn add_document_to_search_engine<P: AsRef<Path>>(&mut self, path: P, watch_id: WatchId) -> Result<()> {
        let document = self.search.make_document(path, watch_id)?;
        let (result_tx, result_rx) = oneshot::channel();
        self.index_writer_command_tx
            .send(IndexWriterCommand::Index { document, result_tx })
            .await
            .map_err(|e| anyhow!(e))?;
        result_rx.await??;
        Ok(())
    }

    async fn delete_document_from_search_engine<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let (result_tx, result_rx) = oneshot::channel();
        self.index_writer_command_tx
            .send(IndexWriterCommand::DeleteByPath {
                path: path.as_ref().to_path_buf(),
                result_tx,
            })
            .await?;
        result_rx.await.map_err(|e| anyhow!(e))??;
        Ok(())
    }

    pub async fn commit_to_search_engine(&mut self) -> Result<()> {
        let (result_tx, result_rx) = oneshot::channel();
        self.index_writer_command_tx
            .send(IndexWriterCommand::Commit { result_tx })
            .await?;
        result_rx.await.map_err(|e| anyhow!(e))??;
        Ok(())
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<DocumentEvent> {
        self.document_event_tx.subscribe()
    }
}
