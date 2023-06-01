use std::path::{Path, PathBuf};

use anyhow::Result;
use futures::{channel::mpsc, SinkExt};
use tracing::instrument;

use crate::documents::{document_gatekeeper::DocumentGatekeeper, Document};

#[derive(Debug)]
pub enum ScanDirectoryEvent {
    FileDetected,
}

pub type ScanDirectoryEventSender = mpsc::Sender<ScanDirectoryEvent>;
pub type ScanDirectoryEventReceiver = mpsc::Receiver<ScanDirectoryEvent>;

#[instrument(skip(scan_tx, event_tx))]
pub async fn scan_directory(
    path: PathBuf,
    document_gatekeeper: DocumentGatekeeper,
    scan_tx: async_channel::Sender<PathBuf>,
    mut event_tx: ScanDirectoryEventSender,
) -> Result<()> {
    let walker = jwalk::WalkDir::new(path).process_read_dir(|_depth, path, _read_dir_state, children| {
        if is_package_dir(path) {
            children.clear();
        }
    });
    for result in walker {
        let span = tracing::trace_span!("WalkDir loop");
        let _guard = span.enter();
        let entry = result?;
        let path = entry.path();
        if document_gatekeeper.is_eligible(&path) {
            scan_tx.send(path.to_path_buf()).await?;
            event_tx.send(ScanDirectoryEvent::FileDetected).await?;
        }
    }

    Ok(())
}

fn is_package_dir<P: AsRef<Path>>(path: P) -> bool {
    use package_dir::*;
    let path = path.as_ref();
    is_npm_package_dir(path)
        || is_bower_package_dir(path)
        || is_python_package_dir(path)
        || is_bundler_package_dir(path)
        || is_composer_package_dir(path)
        || is_chef_cookbook_dir(path)
        || is_cocoapods_pods_dir(path)
}
