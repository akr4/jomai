use std::collections::VecDeque;

use anyhow::Result;
use sqlx::SqlitePool;

use crate::{
    watches::jobs::{Job, JobId, JobType},
    WatchId,
};

mod job_repository;

pub struct JobQueue {
    connection_pool: SqlitePool,
}

impl JobQueue {
    pub fn new(connection_pool: SqlitePool) -> Self {
        Self { connection_pool }
    }

    pub async fn push(&mut self, job_type: JobType, watch_id: WatchId) -> Result<Job> {
        let mut conn = self.connection_pool.acquire().await?;
        let job = job_repository::insert(job_type, watch_id, &mut *conn).await?;
        Ok(job)
    }

    pub async fn pop_for_run(&mut self) -> Result<Option<Job>> {
        let mut conn = self.connection_pool.acquire().await?;
        let jobs = job_repository::get_all(&mut *conn).await?;
        if jobs.is_empty() {
            return Ok(None);
        }
        let job = jobs.first().unwrap();

        let mut conn = self.connection_pool.acquire().await?;
        let job = job.running();
        let job = job_repository::update(&job, &mut *conn)
            .await?
            .ok_or(anyhow::anyhow!("job ({}) not found", job.id))?;
        drop(conn);

        Ok(Some(job))
    }

    pub async fn delete_pending_jobs_by_watch_id(&mut self, watch_id: WatchId) -> Result<()> {
        let mut conn = self.connection_pool.acquire().await?;
        job_repository::delete_pending_jobs_by_watch_id(watch_id, &mut *conn).await?;
        Ok(())
    }

    pub async fn delete_by_job_id(&mut self, job_id: JobId) -> Result<()> {
        let mut conn = self.connection_pool.acquire().await?;
        job_repository::delete(job_id, &mut *conn).await?;
        Ok(())
    }

    pub async fn has_job_for_watch_id(&mut self, watch_id: WatchId) -> Result<bool> {
        let mut conn = self.connection_pool.acquire().await?;
        let jobs = job_repository::get_all(&mut *conn).await?;
        Ok(jobs.iter().any(|job| job.watch_id == watch_id))
    }
}
