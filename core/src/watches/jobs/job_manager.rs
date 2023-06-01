use std::{collections::VecDeque, future::Future};

use anyhow::{anyhow, Result};
use job_queue::JobQueue;
use sqlx::SqlitePool;
use thiserror::Error;
use tokio::{sync::mpsc, task::JoinHandle};
use tracing::{instrument, Instrument};

use crate::{
    documents::DocumentService,
    watches::{
        jobs::{
            delete_watch_job::DeleteWatchJob, job_queue, scan_watch_job::ScanWatchJob, sync_watch_job::SyncWatchJob,
            Job, JobId, JobType,
        },
        watch_repository,
    },
    WatchId,
};

#[derive(Debug)]
enum JobManagerCommand {
    RunJob,
    EnqueueScanWatchJob(WatchId),
    EnqueueDeleteWatchJob(WatchId),
    EnqueueSyncWatchJob(WatchId),
    JobCompleted(JobId),
}

#[derive(Debug, Clone)]
pub struct JobManagerController {
    command_tx: mpsc::UnboundedSender<JobManagerCommand>,
}

impl JobManagerController {
    pub async fn enqueue_scan_watch_path_job(&self, watch_id: WatchId) -> Result<()> {
        self.command_tx.send(JobManagerCommand::EnqueueScanWatchJob(watch_id))?;
        Ok(())
    }

    pub async fn enqueue_delete_watch_job(&self, watch_id: WatchId) -> Result<()> {
        self.command_tx
            .send(JobManagerCommand::EnqueueDeleteWatchJob(watch_id))?;
        Ok(())
    }

    pub async fn enqueue_sync_watch_job(&self, watch_id: WatchId) -> Result<()> {
        self.command_tx.send(JobManagerCommand::EnqueueSyncWatchJob(watch_id))?;
        Ok(())
    }
}

/// Manages status of jobs, launches them.
/// You can enqueue jobs by calling `enqueue_*` functions.
/// JobManager also starts `stopped` jobs at startup (in `run()`).
/// JobManager launches a job at a time.
pub struct JobManager {
    job_queue: JobQueue,
    command_tx: mpsc::UnboundedSender<JobManagerCommand>,
    command_rx: mpsc::UnboundedReceiver<JobManagerCommand>,
    connection_pool: SqlitePool,
    document_service: DocumentService,
    current_job: Option<(Job, JoinHandle<()>)>,
    scan_watch_job: ScanWatchJob,
    delete_watch_job: DeleteWatchJob,
    sync_watch_job: SyncWatchJob,
}

impl JobManager {
    pub fn new(
        connection_pool: SqlitePool,
        document_service: DocumentService,
        scan_watch_job: ScanWatchJob,
        delete_watch_job: DeleteWatchJob,
        sync_watch_job: SyncWatchJob,
    ) -> Self {
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        Self {
            job_queue: JobQueue::new(connection_pool.clone()),
            command_tx,
            command_rx,
            current_job: None,
            connection_pool,
            document_service,
            scan_watch_job,
            delete_watch_job,
            sync_watch_job,
        }
    }

    pub fn controller(&self) -> JobManagerController {
        JobManagerController {
            command_tx: self.command_tx.clone(),
        }
    }

    #[instrument(name = "JobManager::run", level = "info", skip(self))]
    pub async fn run(mut self) -> Result<()> {
        self.enqueue_sync_watch_jobs_for_existing_watches().await?;
        // job starter
        self.command_tx.send(JobManagerCommand::RunJob)?;

        while let Some(command) = self
            .command_rx
            .recv()
            .instrument(tracing::debug_span!("command_rx loop"))
            .await
        {
            let result = match command {
                JobManagerCommand::RunJob => self.run_job().await,
                JobManagerCommand::EnqueueScanWatchJob(watch_id) => self.enqueue_scan_watch_path_job(watch_id).await,
                JobManagerCommand::EnqueueDeleteWatchJob(watch_id) => self.enqueue_delete_watch_job(watch_id).await,
                JobManagerCommand::EnqueueSyncWatchJob(watch_id) => self.enqueue_sync_watch_job(watch_id).await,
                JobManagerCommand::JobCompleted(job_id) => self.complete(job_id).await,
            };

            match result {
                Ok(_) => {}
                Err(e) => {
                    tracing::error!("Error in JobManager.run: {}", e);
                    // go to next command
                    self.command_tx.send(JobManagerCommand::RunJob)?;
                }
            }
        }

        Ok(())
    }

    pub async fn enqueue_scan_watch_path_job(&mut self, watch_id: WatchId) -> Result<()> {
        self.job_queue.push(JobType::ScanWatchPath, watch_id).await?;
        self.command_tx.send(JobManagerCommand::RunJob)?;
        Ok(())
    }

    pub async fn enqueue_delete_watch_job(&mut self, watch_id: WatchId) -> Result<()> {
        self.job_queue.delete_pending_jobs_by_watch_id(watch_id).await?;
        self.cancel_current_job_for_watch_id(watch_id).await?;
        self.job_queue.push(JobType::DeleteWatch, watch_id).await?;
        self.command_tx.send(JobManagerCommand::RunJob)?;
        Ok(())
    }

    pub async fn enqueue_sync_watch_job(&mut self, watch_id: WatchId) -> Result<()> {
        self.cancel_current_job_for_watch_id(watch_id).await?;
        self.job_queue.push(JobType::SyncWatch, watch_id).await?;
        self.command_tx.send(JobManagerCommand::RunJob)?;
        Ok(())
    }

    #[instrument(name = "JobManager::run_job", level = "info", skip(self))]
    async fn run_job(&mut self) -> Result<()> {
        if self.current_job.is_some() {
            tracing::info!("JobManager: job already running");
            return Ok(());
        }

        let job = match self.job_queue.pop_for_run().await? {
            None => {
                return Ok(());
            }
            Some(x) => x,
        };

        let mut conn = self.connection_pool.acquire().await?;
        let watch = watch_repository::find_by_id(job.watch_id, &mut *conn)
            .await?
            .ok_or(anyhow::anyhow!("watch ({}) not found", job.watch_id))?;
        drop(conn);

        let join_handle = match job.job_type {
            JobType::ScanWatchPath => {
                let scan_watch_job = self.scan_watch_job.clone();
                self.spawn_job(job.clone(), scan_watch_job.start(job.id, watch)).await
            }
            JobType::DeleteWatch => {
                let delete_watch_job = self.delete_watch_job.clone();
                self.spawn_job(job.clone(), delete_watch_job.start(watch)).await
            }
            JobType::SyncWatch => {
                let sync_watch_job = self.sync_watch_job.clone();
                self.spawn_job(job.clone(), sync_watch_job.start(job.id, watch)).await
            }
        }?;

        self.current_job = Some((job.clone(), join_handle));

        Ok(())
    }

    #[instrument(name = "JobManager::spawn_job", level = "info", skip(self, future))]
    async fn spawn_job<F>(&mut self, job: Job, future: F) -> Result<JoinHandle<()>>
    where
        F: Future<Output = Result<()>> + Send + 'static,
    {
        let command_tx = self.command_tx.clone();

        let join_handle = tokio::spawn(
            async move {
                match future.await {
                    Ok(_) => {}
                    Err(e) => {
                        tracing::error!("Finished with error: {}", e);
                    }
                }
                command_tx.send(JobManagerCommand::JobCompleted(job.id)).unwrap();
            }
            .instrument(tracing::info_span!("job", job_id = %job.id)),
        );
        Ok(join_handle)
    }

    #[instrument(level = "info", skip(self))]
    async fn cancel_current_job_for_watch_id(&mut self, watch_id: WatchId) -> Result<()> {
        if let Some((job, _)) = self.current_job.as_ref() {
            if job.watch_id != watch_id {
                tracing::warn!("current job is not for watch ({})", watch_id);
                return Ok(());
            }
        }
        let (job, handle) = match self.current_job.take() {
            None => {
                // other task processed the job
                return Ok(());
            }
            Some(x) => x,
        };

        {
            tracing::info!("aborting job ({})", job.id);
            handle.abort();
            handle.await.ok();
            tracing::info!("aborted job ({})", job.id);
        }

        // TODO: tantivy が Unexpected error, empty readers in IndexMerger でクラッシュする問題の回避策
        // 原因は明確ではないが、commit 時に削除が絡んで document がなかったりすると発生するらしく
        // いったんここで commit することで回避はできた。ただし add_documents で foreign key エラーが大量発生するので調査が必要。
        self.document_service.commit_to_search_engine().await?;

        self.job_queue.delete_by_job_id(job.id).await?;

        Ok(())
    }

    async fn enqueue_sync_watch_jobs_for_existing_watches(&mut self) -> Result<()> {
        let mut conn = self.connection_pool.acquire().await?;
        let watches = watch_repository::find_all(&mut *conn).await?;
        drop(conn);

        for watch in watches {
            if self.job_queue.has_job_for_watch_id(watch.id).await? {
                tracing::info!(
                    "Skip enqueuing job for watch ({}) because there is already job.",
                    watch.id,
                );
                continue;
            }
            self.enqueue_sync_watch_job(watch.id).await?;
        }
        Ok(())
    }

    async fn complete(&mut self, job_id: JobId) -> Result<()> {
        self.job_queue.delete_by_job_id(job_id).await?;
        self.current_job = None;
        self.command_tx.send(JobManagerCommand::RunJob)?;
        Ok(())
    }
}
