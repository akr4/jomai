use std::env::VarError;
use std::path::PathBuf;

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(tag = "type", content = "path", rename_all = "snake_case")]
pub enum PathRecommendation {
    Documents(PathBuf),
    Obsidian(PathBuf),
}

pub fn get_path_recommendations() -> Vec<PathRecommendation> {
    let mut paths = Vec::new();
    let home_dir = match std::env::var("HOME") {
        Ok(x) => PathBuf::from(x),
        Err(_) => {
            return paths;
        }
    };

    {
        let documents_dir = home_dir.join("Documents");
        if documents_dir.exists() {
            paths.push(PathRecommendation::Documents(documents_dir));
        }
    }

    {
        let obsidian_dir = home_dir.join("obsidian-vault");
        if obsidian_dir.exists() {
            paths.push(PathRecommendation::Obsidian(obsidian_dir));
        }
    }

    paths
}
