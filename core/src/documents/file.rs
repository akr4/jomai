use std::{fs, path::Path};

use anyhow::Result;

pub struct FileMetadata {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub modified_at: chrono::DateTime<chrono::Utc>,
}

pub fn get_file_metadata<P: AsRef<Path>>(path: P) -> Result<FileMetadata> {
    let metadata = fs::metadata(path)?;
    let created_at = chrono::DateTime::from(metadata.created()?);
    let modified_at = chrono::DateTime::from(metadata.modified()?);
    Ok(FileMetadata {
        created_at,
        modified_at,
    })
}
