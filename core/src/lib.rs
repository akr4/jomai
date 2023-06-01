#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

pub use documents::{SearchResults, Sort};
pub use watches::{jobs::JobReport, Watch, WatchId, WatchState};

pub use crate::core::{AddWatchError, Core, CoreController};

mod assets;
mod core;
mod documents;
mod path_string_normalization;
mod watches;

type DateTime = chrono::DateTime<chrono::Utc>;
