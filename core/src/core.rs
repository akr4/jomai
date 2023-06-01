use std::{fs, path::Path, str::FromStr};

use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, ConnectOptions, SqlitePool};
use thiserror::Error;
use tokio::sync::watch;

use crate::{
    documents,
    documents::{document_gatekeeper::DocumentGatekeeper, IndexWriter},
    watches,
    watches::{
        file_watcher::{FileWatcher, FileWatcherOps},
        jobs,
        jobs::JobManager,
        watch_repository,
    },
};

pub struct Core {
    watch_service: watches::WatchService,
    document_service: documents::DocumentService,
    file_watcher: FileWatcher,
    file_watcher_ops: FileWatcherOps,
    watch_event_handler: watches::WatchEventHandler,
    watch_state_sync: watches::WatchStateSync,
    job_manager: JobManager,
    document_db_writer: documents::DocumentDbCommandProcessor,
    index_writer: IndexWriter,
    connection_pool: SqlitePool,
}

impl Core {
    pub async fn new<P: AsRef<Path>>(app_dir: P) -> Result<(Self, watch::Receiver<watches::WatchState>)> {
        let app_dir = app_dir.as_ref();
        dotenv::dotenv().ok();

        let db_dir = app_dir.join("db");
        if !db_dir.exists() {
            fs::create_dir_all(&db_dir)?;
        }

        let document_gatekeeper = DocumentGatekeeper::new(app_dir.to_path_buf());

        let mut connection_pool_options =
            sqlx::sqlite::SqliteConnectOptions::from_str(format!("sqlite://{}/jomai.db", db_dir.display()).as_str())
                .unwrap()
                .create_if_missing(true)
                .serialized(true);
        connection_pool_options.disable_statement_logging();
        let connection_pool = SqlitePoolOptions::new().connect_with(connection_pool_options).await?;
        sqlx::migrate!().run(&connection_pool).await?;

        let index_dir = app_dir.join("index");
        fs::create_dir_all(&index_dir)?;

        let (document_db_writer, document_db_writer_command_tx) =
            documents::DocumentDbCommandProcessor::new(connection_pool.clone());

        let (search, index_writer) = documents::Search::open_index(&index_dir)?;
        let (index_writer, index_writer_command_tx) = IndexWriter::new(index_writer);
        let document_service = documents::document_service::DocumentService::new(
            document_db_writer_command_tx.clone(),
            index_writer_command_tx,
            search,
        )?;

        let (file_watcher, file_watcher_rx, file_watcher_ops) =
            FileWatcher::make_file_watcher(document_gatekeeper.clone())?;

        let watch_state_sync = watches::WatchStateSync::new(connection_pool.clone(), document_service.clone())?;

        let scan_watch_job = jobs::scan_watch_job::ScanWatchJob::new(
            connection_pool.clone(),
            document_service.clone(),
            document_gatekeeper.clone(),
            watch_state_sync.event_tx(),
        );
        let delete_watch_job = jobs::delete_watch_job::DeleteWatchJob::new(
            connection_pool.clone(),
            watch_state_sync.event_tx(),
            document_service.clone(),
        );
        let sync_watch_job = jobs::sync_watch_job::SyncWatchJob::new(
            document_service.clone(),
            document_gatekeeper.clone(),
            connection_pool.clone(),
            watch_state_sync.event_tx(),
        );

        let job_manager = JobManager::new(
            connection_pool.clone(),
            document_service.clone(),
            scan_watch_job.clone(),
            delete_watch_job.clone(),
            sync_watch_job.clone(),
        );
        let watch_service = watches::WatchService::new(
            job_manager.controller(),
            file_watcher_ops.clone(),
            watch_state_sync.event_tx(),
            watch_state_sync.state_rx(),
        );
        let watch_event_handler =
            watches::WatchEventHandler::new(file_watcher_rx, document_service.clone(), connection_pool.clone());

        let state_rx = watch_state_sync.state_rx();

        Ok((
            Self {
                watch_service,
                document_service,
                file_watcher,
                file_watcher_ops,
                watch_event_handler,
                watch_state_sync,
                job_manager,
                document_db_writer,
                index_writer,
                connection_pool,
            },
            state_rx,
        ))
    }

    pub async fn start(mut self) -> Result<()> {
        tracing::info!("Starting core");
        let mut conn = self.connection_pool.acquire().await?;
        for watch in watch_repository::find_all(&mut conn).await? {
            self.file_watcher_ops.watch_directory(&watch.path).await?;
        }
        drop(conn);

        let mut join_set = tokio::task::JoinSet::new();

        join_set.spawn(self.job_manager.run());
        join_set.spawn(self.file_watcher.run());
        join_set.spawn(self.watch_event_handler.run());
        join_set.spawn(self.watch_state_sync.run());
        join_set.spawn(self.document_db_writer.run());
        join_set.spawn(self.index_writer.run());

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(())) => {}
                Ok(Err(e)) => {
                    tracing::error!("{}", e);
                }
                Err(e) => {
                    tracing::error!("{}", e);
                }
            }
        }

        tracing::info!("Finished core");
        Ok(())
    }

    pub fn controller(&self) -> CoreController {
        CoreController {
            watch_service: self.watch_service.clone(),
            document_service: self.document_service.clone(),
            connection_pool: self.connection_pool.clone(),
        }
    }
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

pub struct CoreController {
    connection_pool: SqlitePool,
    watch_service: watches::WatchService,
    document_service: documents::DocumentService,
}

impl CoreController {
    pub async fn get_all_watches(&self) -> Result<Vec<watches::Watch>> {
        let mut tx = self.connection_pool.begin().await?;
        let results = watch_repository::find_all(&mut tx).await;
        tx.commit().await?;
        results
    }

    pub fn get_watch_state(&self) -> Result<watches::WatchState> {
        self.watch_service.get_state()
    }

    pub async fn add_watch<P: AsRef<Path>>(&mut self, path: P) -> std::result::Result<watches::Watch, AddWatchError> {
        let path = path.as_ref();
        let mut tx = self.connection_pool.begin().await.map_err(|e| anyhow::anyhow!(e))?;
        let results = self.watch_service.add_watch(path, &mut tx).await.map_err(|e| match e {
            watches::AddWatchError::ParentChildRelationship => AddWatchError::ParentChildRelationship,
            watches::AddWatchError::WatchAlreadyExists => AddWatchError::WatchAlreadyExists,
            watches::AddWatchError::Other(e) => AddWatchError::Other(e),
        })?;
        tx.commit().await.map_err(|e| anyhow::anyhow!(e))?;
        Ok(results)
    }

    pub async fn delete_watch<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        let path = path.as_ref();
        let mut tx = self.connection_pool.begin().await?;
        self.watch_service.delete_watch(path, &mut tx).await?;
        tx.commit().await?;
        Ok(())
    }

    pub fn get_all_documents(&self, offset: usize, limit: usize) -> Result<documents::SearchResults> {
        self.document_service.get_all_documents(offset, limit)
    }

    pub fn search_documents(
        &self,
        query: &str,
        tags: &[&str],
        sort: documents::Sort,
        offset: usize,
        limit: usize,
    ) -> Result<documents::SearchResults> {
        self.document_service.search_documents(query, tags, sort, offset, limit)
    }
}
