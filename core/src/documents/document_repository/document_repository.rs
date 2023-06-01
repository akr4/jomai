use std::{fmt::Debug, path::Path};

use anyhow::anyhow;
use chrono::{DateTime, Utc};
use futures::{channel::mpsc, SinkExt, StreamExt};
use sqlx::{error::DatabaseError, SqliteConnection};
use thiserror::Error;
use tokio::sync::oneshot;
use tracing::instrument;

use crate::{
    documents::{document_repository::document_db_command_processor::Command, Document},
    WatchId,
};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("Unique constraint violation")]
    UniqueConstraintViolation,
    #[error("Database error: {0}")]
    Database(#[from] Box<dyn DatabaseError>),
    #[error("sqlx error: {0}")]
    Sqlx(#[from] sqlx::Error),
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

type Result<T> = std::result::Result<T, RepositoryError>;

#[instrument(skip(db_command_tx))]
pub async fn insert(
    path: &Path,
    watch_id: WatchId,
    indexed_at: Option<DateTime<Utc>>,
    mut db_command_tx: mpsc::Sender<Command>,
) -> Result<Document> {
    let (result_tx, result_rx) = oneshot::channel();
    db_command_tx
        .send(Command::Insert {
            path: path.to_path_buf(),
            watch_id,
            indexed_at,
            result_tx,
        })
        .await
        .map_err(|e| anyhow!("Failed to send command: {}", e))?;
    match result_rx.await {
        Ok(Ok(document)) => Ok(document),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(anyhow!("Failed to receive result: {}", e).into()),
    }
}

#[instrument(skip(db_command_tx))]
pub async fn update_by_path<P: AsRef<Path> + Debug>(
    path: P,
    indexed_at: DateTime<Utc>,
    mut db_command_tx: mpsc::Sender<Command>,
) -> Result<Option<Document>> {
    let (result_tx, result_rx) = oneshot::channel();
    db_command_tx
        .send(Command::UpdateByPath {
            path: path.as_ref().to_path_buf(),
            indexed_at,
            result_tx,
        })
        .await
        .map_err(|e| anyhow!("Failed to send command: {}", e))?;
    match result_rx.await {
        Ok(Ok(document)) => Ok(document),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(anyhow!("Failed to receive result: {}", e).into()),
    }
}

#[instrument(skip(db_command_tx))]
pub async fn delete_by_path(path: &Path, mut db_command_tx: mpsc::Sender<Command>) -> Result<Option<Document>> {
    let (result_tx, result_rx) = oneshot::channel();
    db_command_tx
        .send(Command::DeleteByPath {
            path: path.to_path_buf(),
            result_tx,
        })
        .await
        .map_err(|e| anyhow!("Failed to send command: {}", e))?;
    match result_rx.await {
        Ok(Ok(document)) => Ok(document),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(anyhow!("Failed to receive result: {}", e).into()),
    }
}

#[instrument(skip(db_command_tx))]
pub async fn delete_by_watch_id(watch_id: WatchId, mut db_command_tx: mpsc::Sender<Command>) -> Result<()> {
    let (result_tx, result_rx) = oneshot::channel();
    db_command_tx
        .send(Command::DeleteByWatchId { watch_id, result_tx })
        .await
        .map_err(|e| anyhow!("Failed to send command: {}", e))?;
    match result_rx.await {
        Ok(Ok(document)) => Ok(document),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(anyhow!("Failed to receive result: {}", e).into()),
    }
}

#[instrument(skip(db_command_tx))]
pub async fn find_by_path<P: AsRef<Path> + Debug>(
    path: P,
    mut db_command_tx: mpsc::Sender<Command>,
) -> Result<Option<Document>> {
    let (result_tx, result_rx) = oneshot::channel();
    db_command_tx
        .send(Command::FindByPath {
            path: path.as_ref().to_path_buf(),
            result_tx,
        })
        .await
        .map_err(|e| anyhow!("Failed to send command: {}", e))?;
    match result_rx.await {
        Ok(Ok(document)) => Ok(document),
        Ok(Err(e)) => Err(e),
        Err(e) => Err(anyhow!("Failed to receive result: {}", e).into()),
    }
}

#[instrument(skip(conn))]
pub fn find_by_watch_id<'a>(
    watch_id: WatchId,
    conn: &'a mut SqliteConnection,
) -> Result<impl futures::Stream<Item = std::result::Result<Document, anyhow::Error>> + Send + 'a> {
    // query_as! マクロ + fetch() では Copy 可能なパラメーター (watch_id) の参照を返してしまいコンパイルできないので、
    // ここでは query() を使う。
    // https://github.com/launchbadge/sqlx/issues/1151
    let stream = sqlx::query(
        r#"
select id, path, watch_id, created_at, indexed_at
from documents
where watch_id = $1
"#,
    )
    .bind(watch_id)
    .fetch(&mut *conn);
    let documents = stream.map(|result| match result {
        Ok(row) => row.try_into(),
        Err(e) => Err(anyhow!(e)),
    });

    Ok(documents)
}
