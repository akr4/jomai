use std::collections::HashMap;

use anyhow::Result;
use futures::{channel::mpsc, StreamExt};
use itertools::Itertools;
use serde::Serialize;
use sqlx::SqlitePool;
use tokio::sync::watch;
use tracing::instrument;

use crate::{
    documents::{document_service, DocumentService},
    watches,
    watches::{file_watcher::FileWatcherEventReceiver, jobs, watch_repository},
};

#[derive(Debug)]
pub enum WatchEvent {
    AddWatchQueued {
        watch: watches::Watch,
    },
    AddWatchStarted {
        watch: watches::Watch,
    },
    AddWatchProgressed {
        report: jobs::JobReport,
    },
    AddWatchFinished {
        watch: watches::Watch,
        report: jobs::JobReport,
    },
    DeleteWatchQueued {
        watch: watches::Watch,
    },
    DeleteWatchStarted {
        watch: watches::Watch,
    },
    DeleteWatchProgressed {
        report: jobs::JobReport,
    },
    DeleteWatchFinished {
        watch: watches::Watch,
    },
    SyncWatchQueued {
        watch: watches::Watch,
    },
    SyncWatchStarted {
        watch: watches::Watch,
    },
    SyncWatchProgressed {
        report: jobs::JobReport,
    },
    SyncWatchFinished {
        watch: watches::Watch,
        report: jobs::JobReport,
    },
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchState {
    watches: Vec<watches::WatchFull>,
    job_reports: Vec<jobs::JobReport>,
}

pub struct WatchStateSync {
    event_tx: mpsc::UnboundedSender<WatchEvent>,
    event_rx: mpsc::UnboundedReceiver<WatchEvent>,
    state_tx: watch::Sender<WatchState>,
    state_rx: watch::Receiver<WatchState>,
    connection_pool: SqlitePool,
    document_service: DocumentService,
}

impl WatchStateSync {
    pub fn new(connection_pool: SqlitePool, document_service: DocumentService) -> Result<Self> {
        let (event_tx, event_rx) = mpsc::unbounded();
        let (state_tx, state_rx) = watch::channel(WatchState {
            watches: Vec::new(),
            job_reports: Vec::new(),
        });
        Ok(WatchStateSync {
            event_tx,
            event_rx,
            state_tx,
            state_rx,
            connection_pool,
            document_service,
        })
    }

    #[instrument(name = "WatchStateSync::run", level = "info", skip(self))]
    pub async fn run(mut self) -> Result<()> {
        let mut watches: HashMap<watches::WatchId, watches::Watch> = HashMap::new();
        let mut document_count_map: HashMap<watches::WatchId, u32> = HashMap::new();

        let mut document_event_rx = self.document_service.subscribe();

        let mut conn = self.connection_pool.acquire().await?;
        for w in watch_repository::find_all(&mut *conn).await? {
            document_count_map.insert(w.id, self.document_service.count_documents_under_path(&w.path)?);
            watches.insert(w.id, w);
        }
        drop(conn);
        let mut reports: HashMap<watches::WatchId, jobs::JobReport> = HashMap::new();

        // initial state
        self.state_tx.send(WatchState {
            watches: watches
                .values()
                .cloned()
                .map(|w| watches::WatchFull::from_watch(w, 0))
                .collect(),
            job_reports: reports.values().cloned().collect(),
        })?;

        loop {
            tokio::select! {
                Some(event) = self.event_rx.next() => {
                    tracing::trace!("event received: {:?}", event);
                    use WatchEvent::*;
                    match event {
                        AddWatchQueued { watch } => {
                            watches.insert(watch.id, watch);
                        }
                        AddWatchStarted { watch } => {
                            watches.insert(watch.id, watch);
                        }
                        AddWatchProgressed { report } => {
                            reports.insert(report.watch.id, report);
                        }
                        AddWatchFinished { watch, report } => {
                            reports.remove(&watch.id);
                            document_count_map.insert(watch.id, report.progress.done);
                            watches.insert(watch.id, watch);
                        }
                        DeleteWatchQueued { watch } => {
                            watches.insert(watch.id, watch);
                        }
                        DeleteWatchStarted { watch } => {
                            watches.insert(watch.id, watch);
                        }
                        DeleteWatchProgressed { report } => {
                            reports.insert(report.watch.id, report);
                        }
                        DeleteWatchFinished { watch } => {
                            watches.remove(&watch.id);
                            reports.remove(&watch.id);
                        }
                        SyncWatchQueued { watch } => {
                            watches.insert(watch.id, watch);
                        }
                        SyncWatchStarted { watch } => {
                            watches.insert(watch.id, watch);
                        }
                        SyncWatchProgressed { report } => {
                            reports.insert(report.watch.id, report);
                        }
                        SyncWatchFinished { watch, report } => {
                            reports.remove(&watch.id);
                            document_count_map.insert(watch.id, report.progress.done);
                            watches.insert(watch.id, watch);
                        }
                    }
                }

                Ok(event) = document_event_rx.recv() => {
                    use document_service::DocumentEvent::*;
                    match event {
                        DocumentAdded(document) => {
                            if let Some(x) = document_count_map.get_mut(&document.watch_id) {
                                *x += 1;
                            }
                        }
                        DocumentUpdated(_) => {},
                        DocumentDeleted(document) => {
                            if let Some(x) = document_count_map.get_mut(&document.watch_id) {
                                if *x > 0 {
                                    *x -= 1;
                                }
                            }
                        }
                    }
                }
                else => {
                    break;
                }
            }

            self.state_tx.send(WatchState {
                watches: watches
                    .values()
                    .cloned()
                    .map(|w| {
                        let count = *document_count_map.get(&w.id).unwrap_or(&0u32);
                        watches::WatchFull::from_watch(w, count)
                    })
                    .sorted_by(|a, b| a.created_at.cmp(&b.created_at))
                    .collect(),
                job_reports: reports.values().cloned().collect(),
            })?
        }

        Ok(())
    }

    pub fn event_tx(&self) -> mpsc::UnboundedSender<WatchEvent> {
        self.event_tx.clone()
    }

    pub fn state_rx(&self) -> watch::Receiver<WatchState> {
        self.state_rx.clone()
    }
}
