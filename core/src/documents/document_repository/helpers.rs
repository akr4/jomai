use std::path::PathBuf;

use chrono::Utc;
use sqlx::{sqlite::SqliteRow, Row};

use crate::{
    documents::{Document, DocumentId},
    WatchId,
};

pub fn escape_like_pattern<S: AsRef<str>>(s: S) -> String {
    s.as_ref().replace("%", r"\%").replace("_", r"\_").replace(r"\", r"\\")
}

pub struct DocumentRow {
    pub id: i64,
    pub path: String,
    pub watch_id: i64,
    pub created_at: chrono::NaiveDateTime,
    pub indexed_at: Option<chrono::NaiveDateTime>,
}

impl From<DocumentRow> for Document {
    fn from(row: DocumentRow) -> Self {
        Self {
            id: DocumentId(row.id),
            path: PathBuf::from(row.path),
            watch_id: WatchId(row.watch_id),
            created_at: chrono::DateTime::from_utc(row.created_at, Utc),
            indexed_at: row.indexed_at.map(|t| chrono::DateTime::from_utc(t, Utc)),
        }
    }
}

// query_as! マクロの問題の回避のために実装
// https://github.com/launchbadge/sqlx/issues/1151
impl TryFrom<SqliteRow> for Document {
    type Error = anyhow::Error;

    fn try_from(value: SqliteRow) -> std::result::Result<Self, Self::Error> {
        let document = Self {
            id: DocumentId(value.try_get("id")?),
            path: PathBuf::from(value.try_get::<String, &str>("path")?),
            watch_id: WatchId(value.try_get("watch_id")?),
            created_at: chrono::DateTime::from_utc(value.try_get("created_at")?, Utc),
            indexed_at: value
                .try_get::<Option<chrono::NaiveDateTime>, &str>("indexed_at")?
                .map(|t| chrono::DateTime::from_utc(t, Utc)),
        };
        Ok(document)
    }
}
