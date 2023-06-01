#![cfg_attr(all(not(debug_assertions), target_os = "windows"), windows_subsystem = "windows")]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use tauri::Manager;
use tokio::sync::Mutex;
use tracing::instrument;

use jomai_core::CoreController;

use crate::menu::make_menu;
use crate::path_recommendation::PathRecommendation;

mod menu;
mod path_recommendation;
mod tracing_helpers;

type CommandResult<T> = Result<T, String>;

// I need this before tauri::Config is available.
// If you change tauri::Config, you need to change this as well.
const BUNDLE_IDENTIFIER: &str = "app.jomai.jomai";

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install().map_err(|e| anyhow!(e))?;

    let app_dir = tauri::api::path::data_dir().expect("app_dir").join(BUNDLE_IDENTIFIER);
    fs::create_dir_all(&app_dir)?;

    tracing_helpers::init(app_dir.join("logs"))?;

    let (core, watch_state_rx) = jomai_core::Core::new(&app_dir).await?;
    let core_controller = core.controller();

    tokio::spawn(async move {
        match core.start().await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("{}", e);
                std::process::exit(1);
            }
        }
    });

    let app = tauri::Builder::default()
        .menu(make_menu())
        .on_menu_event(|event| match event.menu_item_id() {
            "acknowledgment" => {
                tauri::api::shell::open(
                    &event.window().shell_scope(),
                    "https://jomai.app/acknowledgment.html",
                    None,
                )
                .unwrap();
            }
            id => {
                tracing::debug!("menu event: {}", id);
            }
        })
        .manage(Arc::new(Mutex::new(core_controller)))
        .invoke_handler(tauri::generate_handler![
            get_all_documents,
            search_documents,
            get_watch_state,
            get_all_watches,
            add_watch,
            delete_watch,
            get_containing_folder,
            shutdown,
            get_path_recommendations,
        ])
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .setup(|app| {
            let app_handle = app.app_handle();

            {
                let app_handle = app_handle.clone();
                tokio::spawn(async move {
                    let mut interval = tokio::time::interval(std::time::Duration::from_millis(150));
                    let mut stream = tokio_stream::wrappers::WatchStream::from(watch_state_rx);
                    while let Some(state) = stream.next().await {
                        tracing::trace!("watch state: {:?}", state);
                        app_handle.emit_all("watches", state).unwrap();
                        interval.tick().await;
                    }
                    tracing::info!("watch_state_rx: stream closed");
                });
            }

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("Failed to create Tauri app");

    app.run(
        move |_app_handler, event| {
            if let tauri::RunEvent::ExitRequested { .. } = event {}
        },
    );

    Ok(())
}

#[tauri::command]
#[instrument(skip(core_controller))]
async fn get_all_documents(
    offset: usize,
    limit: usize,
    core_controller: tauri::State<'_, Arc<Mutex<CoreController>>>,
) -> Result<jomai_core::SearchResults, String> {
    tracing::debug!("get_all_documents");
    core_controller
        .lock()
        .await
        .get_all_documents(offset, limit)
        .map_err(|e| format!("failed to get documents: {}", e))
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Sort {
    Relevance,
    Date,
}

impl Into<jomai_core::Sort> for Sort {
    fn into(self) -> jomai_core::Sort {
        match self {
            Sort::Relevance => jomai_core::Sort::Relevance,
            Sort::Date => jomai_core::Sort::Date,
        }
    }
}

#[tauri::command]
#[instrument(skip(core_controller))]
async fn search_documents(
    query: &str,
    tags: Vec<String>,
    sort: Sort,
    offset: usize,
    limit: usize,
    core_controller: tauri::State<'_, Arc<Mutex<CoreController>>>,
) -> Result<jomai_core::SearchResults, String> {
    tracing::debug!("search_documents");
    let tags: Vec<&str> = tags.iter().map(|s| s.as_str()).collect();
    core_controller
        .lock()
        .await
        .search_documents(query, &tags, sort.into(), offset, limit)
        .map_err(|e| format!("failed to search documents: {}", e))
}

#[tauri::command]
#[instrument(skip(core_controller))]
async fn get_all_watches(
    core_controller: tauri::State<'_, Arc<Mutex<CoreController>>>,
) -> Result<Vec<jomai_core::Watch>, String> {
    tracing::debug!("get_all_watches");
    core_controller
        .lock()
        .await
        .get_all_watches()
        .await
        .map_err(|e| format!("failed to get watches: {}", e))
}

#[tauri::command]
#[instrument(skip(core_controller))]
async fn get_watch_state(
    core_controller: tauri::State<'_, Arc<Mutex<CoreController>>>,
) -> Result<jomai_core::WatchState, String> {
    tracing::debug!("get_watch_state");
    core_controller
        .lock()
        .await
        .get_watch_state()
        .map_err(|e| format!("failed to get watches: {:?}", e))
}

#[derive(Debug, Serialize)]
#[serde(tag = "type")]
pub enum AddWatchError {
    #[serde(rename = "parent-child-relationship")]
    ParentChildRelationship,
    #[serde(rename = "watch-already-exists")]
    WatchAlreadyExists,
    #[serde(rename = "other")]
    Other,
}

#[tauri::command]
#[instrument(skip(core_controller))]
async fn add_watch(
    path: &str,
    core_controller: tauri::State<'_, Arc<Mutex<CoreController>>>,
) -> Result<jomai_core::Watch, AddWatchError> {
    tracing::debug!("add_watch");
    let watch = core_controller
        .lock()
        .await
        .add_watch(path)
        .await
        .map_err(|e| match e {
            jomai_core::AddWatchError::ParentChildRelationship => AddWatchError::ParentChildRelationship,
            jomai_core::AddWatchError::WatchAlreadyExists => AddWatchError::WatchAlreadyExists,
            jomai_core::AddWatchError::Other(e) => {
                tracing::error!("failed to add watch: {}", e);
                AddWatchError::Other
            }
        })?;
    Ok(watch)
}

#[tauri::command]
#[instrument(skip(core_controller))]
async fn delete_watch(path: &str, core_controller: tauri::State<'_, Arc<Mutex<CoreController>>>) -> Result<(), String> {
    tracing::debug!("delete_watch");
    core_controller
        .lock()
        .await
        .delete_watch(path)
        .await
        .map_err(|e| format!("failed to delete watch: {}", e))
}

#[tauri::command]
#[instrument]
fn get_containing_folder(path: &str) -> CommandResult<String> {
    tracing::debug!("get_containing_folder");
    let path = Path::new(path);
    if path.is_dir() {
        return Ok(path.display().to_string());
    }
    match path.parent() {
        Some(parent) => Ok(parent.display().to_string()),
        None => Err("failed to get parent".to_string()),
    }
}

#[tauri::command]
#[instrument]
fn shutdown() {
    tracing::debug!("shutdown");
    // TODO: graceful shutdown
    std::process::exit(0);
}

#[tauri::command]
#[instrument]
fn get_path_recommendations() -> CommandResult<Vec<PathRecommendation>> {
    tracing::debug!("get_path_recommendations");
    Ok(path_recommendation::get_path_recommendations())
}
