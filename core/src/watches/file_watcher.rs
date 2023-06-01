use std::path::{Path, PathBuf};

use anyhow::Result;
use futures::{channel::mpsc, SinkExt, StreamExt};
use notify::{
    event::{CreateKind, ModifyKind, RemoveKind, RenameMode},
    recommended_watcher, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use tracing::instrument;

use crate::{documents::document_gatekeeper::DocumentGatekeeper, watches::FileWatcherEvent};

pub enum Operation {
    WatchDirectory(PathBuf),
    UnwatchDirectory(PathBuf),
}

#[derive(Clone)]
pub struct FileWatcherOps {
    operation_tx: mpsc::Sender<Operation>,
}

impl FileWatcherOps {
    pub async fn watch_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        use Operation::*;
        self.operation_tx
            .send(WatchDirectory(path.as_ref().to_path_buf()))
            .await?;
        Ok(())
    }

    pub async fn unwatch_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        use Operation::*;
        self.operation_tx
            .send(UnwatchDirectory(path.as_ref().to_path_buf()))
            .await?;
        Ok(())
    }
}

/// filesystem event watcher that translates notify crates' events to this app's and sends them to a channel.
pub struct FileWatcher {
    watcher: RecommendedWatcher,
    document_gatekeeper: DocumentGatekeeper,
    notify_rx: mpsc::Receiver<Result<notify::Event, notify::Error>>,
    event_tx: tokio::sync::broadcast::Sender<FileWatcherEvent>,
    operation_rx: mpsc::Receiver<Operation>,
}

pub type FileWatcherEventReceiver = tokio::sync::broadcast::Receiver<FileWatcherEvent>;

impl FileWatcher {
    pub fn make_file_watcher(
        document_gatekeeper: DocumentGatekeeper,
    ) -> Result<(Self, FileWatcherEventReceiver, FileWatcherOps)> {
        let (event_tx, event_rx) = tokio::sync::broadcast::channel(100);
        let (operation_tx, operation_rx) = mpsc::channel(100);
        let (mut notify_tx, notify_rx) = mpsc::channel(100);
        let watcher = recommended_watcher(move |res| {
            futures::executor::block_on(async {
                // TODO: get rid of unwrap
                notify_tx.send(res).await.unwrap();
            })
        })?;
        Ok((
            Self {
                watcher,
                document_gatekeeper,
                notify_rx,
                event_tx,
                operation_rx,
            },
            event_rx,
            FileWatcherOps { operation_tx },
        ))
    }

    pub fn watch_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        tracing::info!("Watching directory: {}", path.display());
        self.watcher.watch(path, RecursiveMode::Recursive)?;
        Ok(())
    }

    pub fn unwatch_directory<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        tracing::info!("Unwatching directory: {}", path.display());
        self.watcher.unwatch(path)?;
        Ok(())
    }

    #[instrument(name = "FileWatcher::run", level = "info", skip(self))]
    pub async fn run(mut self) -> Result<()> {
        while let Ok(_) = self.process_rxs().await {}
        Ok(())
    }

    async fn process_rxs(&mut self) -> Result<()> {
        tokio::select! {
            result = self.notify_rx.next() => {
                if let Some(result) = result {
                    let event = result?;
                    tracing::trace!("file watcher: event: {:?}", event);
                    if let Some(event_to_send) = self.convert_event(&event) {
                        self.event_tx.send(event_to_send)?;
                    }
                    return Ok(());
                }
                Err(anyhow::anyhow!("notify channel is closed."))
            }
            result = self.operation_rx.next() => {
                if let Some(result) = result {
                    match result {
                        Operation::WatchDirectory(path) => self.watch_directory(path)?,
                        Operation::UnwatchDirectory(path) => self.unwatch_directory(path)?,
                    }
                    return Ok(());
                }
                Err(anyhow::anyhow!("operation channel is closed."))
            }
        }
    }

    fn convert_event(&self, event: &notify::Event) -> Option<FileWatcherEvent> {
        if let Some(info) = event.info() {
            tracing::debug!("event.info={}", info);
        }
        match &event.kind {
            EventKind::Any => {}
            EventKind::Access(_) => {}
            EventKind::Create(kind) => match kind {
                CreateKind::Any => {}
                CreateKind::File => {
                    let path = event.paths.first().unwrap();
                    if self.document_gatekeeper.is_eligible(path) {
                        return Some(FileWatcherEvent::Created(path.to_path_buf()));
                    }
                }
                CreateKind::Folder => {}
                CreateKind::Other => {}
            },
            EventKind::Modify(kind) => match kind {
                ModifyKind::Any => {}
                ModifyKind::Data(_) | ModifyKind::Metadata(_) => {
                    // There is no Data event when some app (e.g. Typora) saves a file.
                    // In that case, there is a Metadata event instead.
                    let path = event.paths.first().unwrap();
                    if self.document_gatekeeper.is_eligible(path) {
                        return Some(FileWatcherEvent::Modified(path.to_path_buf()));
                    }
                }
                ModifyKind::Name(mode) => {
                    use RenameMode::*;
                    match mode {
                        Any => {
                            if event.paths.len() == 1 {
                                let path = event.paths.first().unwrap();
                                if path.exists() {
                                    if self.document_gatekeeper.is_eligible(path) {
                                        return Some(FileWatcherEvent::Created(path.to_path_buf()));
                                    }
                                } else {
                                    if self.document_gatekeeper.is_eligible_if_file_exists(path) {
                                        return Some(FileWatcherEvent::Removed(path.to_path_buf()));
                                    }
                                }
                            } else {
                                tracing::warn!(
                                    "Unexpected paths length {} for RenameMode::Any, {:?}",
                                    event.paths.len(),
                                    event
                                );
                            }
                        }
                        To | From | Both | Other => {
                            tracing::warn!("Unexpected RenameMode {:?}", mode);
                        }
                    }
                }
                ModifyKind::Other => {}
            },
            EventKind::Remove(remove_kind) => match remove_kind {
                RemoveKind::Any => {}
                RemoveKind::File => {
                    let path = event.paths.first().unwrap();
                    if self.document_gatekeeper.is_eligible_if_file_exists(path) {
                        return Some(FileWatcherEvent::Removed(path.to_path_buf()));
                    }
                }
                RemoveKind::Folder => {}
                RemoveKind::Other => {}
            },
            EventKind::Other => {}
        }

        None
    }
}
