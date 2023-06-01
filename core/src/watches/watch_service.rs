use std::path::Path;

use anyhow::Result;
use futures::{channel::mpsc, SinkExt, TryFutureExt};
use sqlx::SqliteConnection;
use thiserror::Error;
use tokio::sync::watch;

use crate::watches::{
    file_watcher::FileWatcherOps, jobs::JobManagerController, path_helpers::is_parent, watch_repository, Watch,
    WatchEvent, WatchState, WatchStatus,
};

#[derive(Clone)]
pub struct WatchService {
    job_manager_controller: JobManagerController,
    file_watcher_ops: FileWatcherOps,
    watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
    watch_state_rx: watch::Receiver<WatchState>,
}

#[derive(Error, Debug)]
pub enum AddWatchError {
    #[error("Watch path is in a parent-children relationship with an existing watch")]
    ParentChildRelationship,
    #[error("Watch already exists")]
    WatchAlreadyExists,
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl WatchService {
    pub fn new(
        job_manager_controller: JobManagerController,
        file_watcher_ops: FileWatcherOps,
        watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
        watch_state_rx: watch::Receiver<WatchState>,
    ) -> Self {
        Self {
            job_manager_controller,
            file_watcher_ops,
            watch_event_tx,
            watch_state_rx,
        }
    }

    pub fn get_state(&self) -> Result<WatchState> {
        Ok(self.watch_state_rx.borrow().clone())
    }

    pub async fn add_watch<P: AsRef<Path>>(
        &mut self,
        path: P,
        conn: &mut SqliteConnection,
    ) -> Result<Watch, AddWatchError> {
        let path = path.as_ref();

        let all_watches = watch_repository::find_all(&mut *conn).await?;

        if all_watches.iter().any(|w| w.path == path) {
            return Err(AddWatchError::WatchAlreadyExists);
        }

        let parent_or_child = all_watches
            .iter()
            .find(|w| is_parent(&w.path, path) || is_parent(path, &w.path));
        if parent_or_child.is_some() {
            return Err(AddWatchError::ParentChildRelationship);
        }

        let watch = watch_repository::insert(path, &mut *conn).await?;
        self.file_watcher_ops.watch_directory(path).await?;
        self.job_manager_controller
            .enqueue_scan_watch_path_job(watch.id)
            .map_err(|e| anyhow::anyhow!(e))
            .await?;
        self.watch_event_tx
            .send(WatchEvent::AddWatchQueued { watch: watch.clone() })
            .map_err(|e| anyhow::anyhow!(e))
            .await?;
        Ok(watch)
    }

    pub async fn delete_watch<P: AsRef<Path>>(&mut self, path: P, conn: &mut SqliteConnection) -> Result<()> {
        let path = path.as_ref();
        let watch = match watch_repository::find_by_path(path, &mut *conn).await? {
            None => {
                return Ok(());
            }
            Some(x) => x,
        };
        watch_repository::update(watch.id, WatchStatus::Deleting, &mut *conn).await?;
        self.file_watcher_ops.unwatch_directory(path).await?;
        self.job_manager_controller.enqueue_delete_watch_job(watch.id).await?;
        self.watch_event_tx
            .send(WatchEvent::DeleteWatchQueued {
                watch: watch.deleting(),
            })
            .await?;
        Ok(())
    }
}
