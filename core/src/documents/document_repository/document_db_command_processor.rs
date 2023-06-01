use std::path::{Path, PathBuf};

use anyhow::Result;
use chrono::{DateTime, Utc};
use futures::{channel::mpsc, StreamExt};
use sqlx::{sqlite::SqliteQueryResult, SqliteConnection, SqlitePool};
use tokio::sync::oneshot;
use tracing::instrument;

use super::{document_repository::RepositoryError, helpers::DocumentRow};
use crate::{documents::Document, path_string_normalization::PathStringNormalizationExt, WatchId};

pub enum Command {
    Insert {
        watch_id: WatchId,
        path: PathBuf,
        indexed_at: Option<DateTime<Utc>>,
        result_tx: oneshot::Sender<Result<Document, RepositoryError>>,
    },
    UpdateByPath {
        path: PathBuf,
        indexed_at: DateTime<Utc>,
        result_tx: oneshot::Sender<Result<Option<Document>, RepositoryError>>,
    },
    DeleteByPath {
        path: PathBuf,
        result_tx: oneshot::Sender<Result<Option<Document>, RepositoryError>>,
    },
    DeleteByWatchId {
        watch_id: WatchId,
        result_tx: oneshot::Sender<Result<(), RepositoryError>>,
    },
    FindByPath {
        path: PathBuf,
        result_tx: oneshot::Sender<Result<Option<Document>, RepositoryError>>,
    },
}

pub struct DocumentDbCommandProcessor {
    connection_pool: SqlitePool,
    command_rx: mpsc::Receiver<Command>,
}

impl DocumentDbCommandProcessor {
    pub fn new(connection_pool: SqlitePool) -> (Self, mpsc::Sender<Command>) {
        let (command_tx, command_rx) = mpsc::channel(100_000);
        (
            Self {
                connection_pool,
                command_rx,
            },
            command_tx,
        )
    }

    #[instrument(name = "DocumentDbCommandProcessor::run", level = "info", skip(self))]
    pub async fn run(mut self) -> Result<()> {
        loop {
            while let Some(command) = self.command_rx.next().await {
                match command {
                    Command::Insert {
                        watch_id,
                        path,
                        indexed_at,
                        result_tx,
                    } => {
                        let mut conn = self.connection_pool.acquire().await?;
                        let result = insert(path.as_path(), watch_id, indexed_at, &mut conn).await;
                        match result_tx.send(result) {
                            Ok(_) => {}
                            Err(_) => {
                                tracing::error!("Failed to send result");
                            }
                        }
                    }
                    Command::UpdateByPath {
                        path,
                        indexed_at,
                        result_tx,
                    } => {
                        let mut conn = self.connection_pool.acquire().await?;
                        let result = update_by_path(path.as_path(), indexed_at, &mut conn).await;
                        match result_tx.send(result) {
                            Ok(_) => {}
                            Err(_) => {
                                tracing::error!("Failed to send result");
                            }
                        }
                    }
                    Command::DeleteByPath { path, result_tx } => {
                        let mut conn = self.connection_pool.acquire().await?;
                        let result = delete_by_path(path.as_path(), &mut conn).await;
                        match result_tx.send(result) {
                            Ok(_) => {}
                            Err(_) => {
                                tracing::error!("Failed to send result");
                            }
                        }
                    }
                    Command::DeleteByWatchId { watch_id, result_tx } => {
                        let mut conn = self.connection_pool.acquire().await?;
                        let result = delete_by_watch_id(watch_id, &mut conn).await;
                        match result_tx.send(result) {
                            Ok(_) => {}
                            Err(_) => {
                                tracing::error!("Failed to send result");
                            }
                        }
                    }
                    Command::FindByPath { path, result_tx } => {
                        let mut conn = self.connection_pool.acquire().await?;
                        let result = find_by_path(path.as_path(), &mut conn).await;
                        match result_tx.send(result) {
                            Ok(_) => {}
                            Err(_) => {
                                tracing::error!("Failed to send result");
                            }
                        }
                    }
                };
            }
        }
    }
}

async fn insert<P: AsRef<Path>>(
    path: P,
    watch_id: WatchId,
    indexed_at: Option<DateTime<Utc>>,
    conn: &mut SqliteConnection,
) -> Result<Document, RepositoryError> {
    let path_string = path.as_ref().to_normalized_path_string();
    let result: sqlx::Result<SqliteQueryResult> = sqlx::query!(
        r#"
insert into documents (path, watch_id, indexed_at) values ($1, $2, $3)
"#,
        path_string,
        watch_id,
        indexed_at,
    )
    .execute(&mut *conn)
    .await;

    match result {
        Ok(_) => Ok(find_by_path(path, &mut *conn).await?.unwrap()),
        Err(e) => match e {
            sqlx::Error::Database(e) => {
                // https://www.sqlite.org/rescode.html#constraint_unique
                if let Some(e) = e.code() {
                    if e == "2067" {
                        return Err(RepositoryError::UniqueConstraintViolation);
                    }
                }
                Err(RepositoryError::Database(e))
            }
            _ => Err(RepositoryError::Sqlx(e)),
        },
    }
}

async fn update_by_path<P: AsRef<Path>>(
    path: P,
    indexed_at: DateTime<Utc>,
    conn: &mut SqliteConnection,
) -> Result<Option<Document>, RepositoryError> {
    let path_string = path.as_ref().to_normalized_path_string();
    sqlx::query!(
        r#"
update documents
set indexed_at = $1
where path = $2
"#,
        indexed_at,
        path_string,
    )
    .execute(&mut *conn)
    .await?;

    Ok(find_by_path(path, &mut *conn).await?)
}

async fn delete_by_path<P: AsRef<Path>>(
    path: P,
    conn: &mut SqliteConnection,
) -> Result<Option<Document>, RepositoryError> {
    let path = path.as_ref();
    let document = find_by_path(path, &mut *conn).await?;
    if document.is_none() {
        return Ok(None);
    }
    let path_string = path.to_normalized_path_string();
    sqlx::query!(
        r#"
delete from documents where path = $1
"#,
        path_string,
    )
    .execute(&mut *conn)
    .await?;

    Ok(document)
}

async fn delete_by_watch_id(watch_id: WatchId, conn: &mut SqliteConnection) -> Result<(), RepositoryError> {
    sqlx::query!(
        r#"
delete from documents where watch_id = $1
"#,
        watch_id.0,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

async fn find_by_path<P: AsRef<Path>>(
    path: P,
    conn: &mut SqliteConnection,
) -> Result<Option<Document>, RepositoryError> {
    let path_string = path.as_ref().to_normalized_path_string();
    let row = sqlx::query_as!(
        DocumentRow,
        r#"
select id, path, watch_id, created_at, indexed_at
from documents
where path = $1
"#,
        path_string,
    )
    .fetch_optional(&mut *conn)
    .await?;

    Ok(row.map(|r| r.into()))
}
