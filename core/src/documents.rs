use std::path::PathBuf;

pub use document_repository::document_db_command_processor::DocumentDbCommandProcessor;
pub use document_service::DocumentService;
pub use search::{
    index_writer::{IndexWriter, IndexWriterCommand},
    Search, SearchResults, Sort,
};
use serde::Serialize;

use crate::WatchId;

pub mod document_gatekeeper;
mod document_repository;
pub mod document_service;
mod file;
mod markdown;
mod search;

#[derive(Serialize, Debug, Copy, Clone, Eq, PartialEq, Hash, sqlx::Type)]
#[sqlx(transparent)]
pub struct DocumentId(i64);

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Document {
    pub id: DocumentId,
    pub path: PathBuf,
    pub watch_id: WatchId,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub indexed_at: Option<chrono::DateTime<chrono::Utc>>,
}
