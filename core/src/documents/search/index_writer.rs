use std::{
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
};

use anyhow::Result;
use futures::{channel::mpsc, StreamExt};
use tantivy::{Document, Term};
use tokio::sync::oneshot;
use tracing::instrument;

use crate::{documents::search::schema::AppSchema, path_string_normalization::PathStringNormalizationExt, WatchId};

pub enum IndexWriterCommand {
    Index {
        document: Document,
        result_tx: oneshot::Sender<Result<()>>,
    },
    DeleteByPath {
        path: PathBuf,
        result_tx: oneshot::Sender<Result<()>>,
    },
    DeleteByWatchId {
        watch_id: WatchId,
        result_tx: oneshot::Sender<Result<()>>,
    },
    Commit {
        result_tx: oneshot::Sender<Result<()>>,
    },
}

pub struct IndexWriter {
    index_writer: tantivy::IndexWriter,
    command_rx: mpsc::Receiver<IndexWriterCommand>,
}

impl IndexWriter {
    pub fn new(index_writer: tantivy::IndexWriter) -> (Self, mpsc::Sender<IndexWriterCommand>) {
        let (command_tx, command_rx) = mpsc::channel(100_000);
        (
            Self {
                index_writer,
                command_rx,
            },
            command_tx,
        )
    }

    #[instrument(name = "IndexWriter::run", level = "info", skip(self))]
    pub async fn run(mut self) -> Result<()> {
        tracing::info!("Index writer started");

        while let Some(command) = self.command_rx.next().await {
            match command {
                IndexWriterCommand::Index { document, result_tx } => match self.index_writer.add_document(document) {
                    Ok(_) => match result_tx.send(Ok(())) {
                        Ok(_) => {}
                        Err(_) => {
                            tracing::error!("Failed to send result to channel");
                        }
                    },
                    Err(e) => match result_tx.send(Err(e.into())) {
                        Ok(_) => {}
                        Err(_) => {
                            tracing::error!("Failed to send result to channel");
                        }
                    },
                },
                IndexWriterCommand::DeleteByPath { path, result_tx } => {
                    let result = self.delete_document(&path);
                    match result_tx.send(result) {
                        Ok(_) => {}
                        Err(_) => {
                            tracing::error!("Failed to send result to channel");
                        }
                    }
                }
                IndexWriterCommand::DeleteByWatchId { watch_id, result_tx } => {
                    let result = self.delete_documents_by_watch_id(watch_id);
                    match result_tx.send(result) {
                        Ok(_) => {}
                        Err(_) => {
                            tracing::error!("Failed to send result to channel");
                        }
                    }
                }
                IndexWriterCommand::Commit { result_tx } => {
                    let result = self.commit();
                    match result_tx.send(result) {
                        Ok(_) => {}
                        Err(_) => {
                            tracing::error!("Failed to send result to channel");
                        }
                    }
                }
            }
        }

        tracing::info!("Index writer stopped");
        Ok(())
    }

    fn delete_document<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        let path_field = AppSchema::new(self.index_writer.index().schema()).path();
        self.index_writer
            .delete_term(Term::from_field_text(path_field, &path.to_normalized_path_string()));
        Ok(())
    }

    fn delete_documents_by_watch_id(&self, watch_id: WatchId) -> Result<()> {
        let watch_id_field = AppSchema::new(self.index_writer.index().schema()).watch_id();
        self.index_writer
            .delete_term(Term::from_field_i64(watch_id_field, watch_id.0));
        Ok(())
    }

    fn commit(&mut self) -> Result<()> {
        self.index_writer.commit()?;
        Ok(())
    }
}
