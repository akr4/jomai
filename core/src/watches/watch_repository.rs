use std::path::Path;

use anyhow::Result;
use chrono::Utc;
use sqlx::SqliteConnection;

use crate::{
    path_string_normalization::PathStringNormalizationExt,
    watches::{Watch, WatchId, WatchStatus},
};

pub async fn insert<P: AsRef<Path>>(path: P, conn: &mut SqliteConnection) -> Result<Watch> {
    let path_string = path.as_ref().to_normalized_path_string();
    sqlx::query!(
        r"
 insert into watches (path, status) values ($1, 'adding')
",
        path_string,
    )
    .execute(&mut *conn)
    .await?;

    Ok(find_by_path(path, &mut *conn).await?.unwrap())
}

pub async fn update(watch_id: WatchId, status: WatchStatus, conn: &mut SqliteConnection) -> Result<Option<Watch>> {
    sqlx::query!(
        r"
update watches set status = $1 where id = $2
",
        status,
        watch_id,
    )
    .execute(&mut *conn)
    .await?;

    Ok(find_by_id(watch_id, &mut *conn).await?)
}

pub async fn delete(id: WatchId, conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query!(
        r"
delete from watches where id = $1
",
        id,
    )
    .execute(conn)
    .await?;

    Ok(())
}

pub async fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Watch>> {
    let watches = sqlx::query_as!(
        WatchRow,
        r"
select * from watches order by created_at
"
    )
    .fetch_all(&mut *conn)
    .await?
    .into_iter()
    .map(|r| r.into())
    .collect();

    Ok(watches)
}

pub async fn find_by_id(id: WatchId, conn: &mut SqliteConnection) -> Result<Option<Watch>> {
    let watch = sqlx::query_as!(
        WatchRow,
        r"
select id, path, status, created_at
from watches
where id = $1
",
        id,
    )
    .fetch_optional(conn)
    .await?;

    Ok(watch.map(|r| r.into()))
}

pub async fn find_by_path<P: AsRef<Path>>(path: P, conn: &mut SqliteConnection) -> Result<Option<Watch>> {
    let path_string = path.as_ref().to_normalized_path_string();
    let watch = sqlx::query_as!(
        WatchRow,
        r"
select id, path, status, created_at
from watches
where path = $1
",
        path_string,
    )
    .fetch_optional(conn)
    .await?;

    Ok(watch.map(|r| r.into()))
}

pub async fn find_containing_path<P: AsRef<Path>>(path: P, conn: &mut SqliteConnection) -> Result<Option<Watch>> {
    let path_string = path.as_ref().to_normalized_path_string();
    let watch = sqlx::query_as!(
        WatchRow,
        r"
select id, path, status, created_at
from watches
where $1 like path || '%'
",
        path_string,
    )
    .fetch_optional(&mut *conn)
    .await?;

    Ok(watch.map(|r| r.into()))
}

struct WatchRow {
    id: i64,
    path: String,
    status: String,
    created_at: chrono::NaiveDateTime,
}

impl From<WatchRow> for Watch {
    fn from(row: WatchRow) -> Self {
        Self {
            id: WatchId(row.id),
            path: row.path.into(),
            status: row.status.parse().unwrap(),
            created_at: chrono::DateTime::from_utc(row.created_at, Utc),
        }
    }
}
