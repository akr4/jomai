use anyhow::Result;
use sqlx::SqliteConnection;

use crate::{
    watches::jobs::{Job, JobId, JobType},
    WatchId,
};

pub async fn insert(job_type: JobType, watch_id: WatchId, conn: &mut SqliteConnection) -> Result<Job> {
    let now = chrono::Utc::now();
    let id = sqlx::query!(
        r"
insert into jobs
(job_type, watch_id, status, created_at, started_at)
values ($1, $2, 'pending', $3, $4)
returning id
",
        job_type,
        watch_id,
        now,
        now,
    )
    .fetch_one(&mut *conn)
    .await?
    .id;

    Ok(find_job_by_id(JobId(id), &mut *conn).await?.unwrap())
}

pub async fn update(job: &Job, conn: &mut SqliteConnection) -> Result<Option<Job>> {
    sqlx::query!(
        r"
update jobs
set status = $2,
started_at = $3
where id = $1
",
        job.id,
        job.status,
        job.started_at,
    )
    .execute(&mut *conn)
    .await?;

    Ok(find_job_by_id(job.id, &mut *conn).await?)
}

pub async fn delete(id: JobId, conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query!(r"delete from jobs where id = $1", id,)
        .execute(&mut *conn)
        .await?;

    Ok(())
}

pub async fn delete_pending_jobs_by_watch_id(watch_id: WatchId, conn: &mut SqliteConnection) -> Result<()> {
    sqlx::query!(
        r"
delete from jobs
where watch_id = $1
  and status = 'pending'
",
        watch_id,
    )
    .execute(&mut *conn)
    .await?;

    Ok(())
}

pub async fn get_all(conn: &mut SqliteConnection) -> Result<Vec<Job>> {
    let jobs = sqlx::query_as!(JobRow, r"select * from jobs order by created_at")
        .fetch_all(&mut *conn)
        .await?
        .into_iter()
        .map(|r| r.into())
        .collect();

    Ok(jobs)
}

pub async fn find_job_by_id(id: JobId, conn: &mut SqliteConnection) -> Result<Option<Job>> {
    let job = sqlx::query_as!(JobRow, "select * from jobs where id = $1", id)
        .fetch_optional(&mut *conn)
        .await?;

    Ok(job.map(|j| j.into()))
}

struct JobRow {
    id: i64,
    watch_id: i64,
    job_type: String,
    status: String,
    created_at: chrono::NaiveDateTime,
    started_at: chrono::NaiveDateTime,
}

impl From<JobRow> for Job {
    fn from(row: JobRow) -> Self {
        Self {
            id: JobId(row.id),
            watch_id: row.watch_id.into(),
            job_type: row.job_type.parse().unwrap(),
            status: row.status.parse().unwrap(),
            created_at: chrono::DateTime::from_utc(row.created_at, chrono::Utc),
            started_at: chrono::DateTime::from_utc(row.created_at, chrono::Utc),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path;

    use anyhow::Result;
    use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

    use super::*;
    use crate::watches::watch_repository;

    async fn init() -> Result<(tempfile::TempDir, SqlitePool)> {
        let db_dir = tempfile::tempdir()?;
        let connection = SqlitePoolOptions::new()
            .connect(format!("sqlite://{}/jomai.db?mode=rwc", db_dir.path().display()).as_str())
            .await?;
        sqlx::migrate!().run(&connection).await?;
        Ok((db_dir, connection))
    }

    #[tokio::test]
    async fn test_insert() -> Result<()> {
        let (_db_dir, pool) = init().await?;
        let mut conn = pool.acquire().await?;

        let path = path::PathBuf::from("/foo");
        let watch = watch_repository::insert(&path, &mut *conn).await?;
        let job = insert(JobType::ScanWatchPath, watch.id, &mut *conn).await?;

        assert_eq!(job.id, JobId(1));
        Ok(())
    }
}
