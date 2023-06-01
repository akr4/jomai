use std::{
    fmt::{Display, Formatter},
    path,
    str::FromStr,
};

use serde::Serialize;
pub use watch_event_handler::WatchEventHandler;
pub use watch_service::{AddWatchError, WatchService};
pub use watch_state_sync::{WatchEvent, WatchState, WatchStateSync};

pub mod file_watcher;
pub mod jobs;
mod path_helpers;
mod watch_event_handler;
pub mod watch_repository;
mod watch_service;
mod watch_state_sync;

#[derive(Serialize, Debug, Copy, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct WatchId(pub(crate) i64);

impl From<i64> for WatchId {
    fn from(value: i64) -> Self {
        WatchId(value)
    }
}

impl Display for WatchId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Debug, Clone, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(rename_all = "lowercase")]
pub enum WatchStatus {
    Active,
    Adding,
    Deleting,
}

impl FromStr for WatchStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "active" => Ok(WatchStatus::Active),
            "adding" => Ok(WatchStatus::Adding),
            "deleting" => Ok(WatchStatus::Deleting),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Watch {
    pub id: WatchId,
    pub path: path::PathBuf,
    pub status: WatchStatus,
    #[serde(rename = "createdAt")]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Watch {
    pub fn active(&self) -> Self {
        Self {
            status: WatchStatus::Active,
            ..self.clone()
        }
    }

    pub fn deleting(&self) -> Self {
        Self {
            status: WatchStatus::Deleting,
            ..self.clone()
        }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WatchFull {
    // members from Watch
    pub id: WatchId,
    pub path: path::PathBuf,
    pub status: WatchStatus,
    pub created_at: chrono::DateTime<chrono::Utc>,
    // additional members
    pub document_count: u32,
}

impl WatchFull {
    pub fn from_watch(watch: Watch, document_count: u32) -> Self {
        Self {
            id: watch.id,
            path: watch.path,
            status: watch.status,
            created_at: watch.created_at,
            document_count,
        }
    }
}

#[derive(Debug, Clone)]
pub enum FileWatcherEvent {
    Created(path::PathBuf),
    Modified(path::PathBuf),
    Removed(path::PathBuf),
}

impl FileWatcherEvent {
    pub fn get_first_path(&self) -> &path::Path {
        match self {
            FileWatcherEvent::Created(path) => path,
            FileWatcherEvent::Modified(path) => path,
            FileWatcherEvent::Removed(path) => path,
        }
    }
}
