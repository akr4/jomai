use std::{fmt, fmt::Formatter, str::FromStr};

pub use job_manager::{JobManager, JobManagerController};
use serde::{Deserialize, Serialize};

use crate::{DateTime, Watch, WatchId};

pub mod delete_watch_job;
mod job_manager;
pub mod job_queue;
mod parallelism;
pub mod scan_watch_job;
pub mod sync_watch_job;
mod tasks;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct JobId(i64);

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: JobId,
    pub job_type: JobType,
    pub watch_id: WatchId,
    pub status: JobStatus,
    pub created_at: DateTime,
    pub started_at: DateTime,
}

impl Job {
    pub fn pending(&self) -> Self {
        Job {
            status: JobStatus::Pending,
            ..self.clone()
        }
    }

    pub fn running(&self) -> Self {
        Job {
            status: JobStatus::Running,
            ..self.clone()
        }
    }

    pub fn started(&self) -> Self {
        Job {
            status: JobStatus::Running,
            started_at: chrono::Utc::now(),
            ..self.clone()
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum JobStatus {
    Pending,
    Running,
    Finished,
}

impl FromStr for JobStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pending" => Ok(JobStatus::Pending),
            "running" => Ok(JobStatus::Running),
            "finished" => Ok(JobStatus::Finished),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Serialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum JobType {
    ScanWatchPath,
    DeleteWatch,
    SyncWatch,
}

impl FromStr for JobType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "scan_watch_path" => Ok(JobType::ScanWatchPath),
            "delete_watch" => Ok(JobType::DeleteWatch),
            "sync_watch" => Ok(JobType::SyncWatch),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct JobProgress {
    pub done: u32,
    pub failed: u32,
    pub total: u32,
}

impl JobProgress {
    pub fn new(done: u32, failed: u32, total: u32) -> JobProgress {
        JobProgress { done, failed, total }
    }
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReport {
    pub watch: Watch,
    pub progress: JobProgress,
    pub job_type: JobType,
    pub status: JobStatus,
}
