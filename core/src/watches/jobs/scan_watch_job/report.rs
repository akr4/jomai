use anyhow::Result;
use futures::{channel::mpsc, SinkExt, StreamExt};
use tracing::instrument;

use super::super::tasks::{add_documents::AddDocumentsEvent, scan_directory::ScanDirectoryEvent};
use crate::{
    watches::{
        jobs::{
            tasks::{add_documents::AddDocumentsEventReceiver, scan_directory::ScanDirectoryEventReceiver},
            JobProgress, JobStatus, JobType,
        },
        WatchEvent,
    },
    JobReport, Watch,
};

pub struct ReporterResult {
    pub done: u32,
    pub failed: u32,
    pub total: u32,
}

#[instrument(
    name = "scan_watch_job::report",
    level = "info",
    skip(registrar_event_rx, scanner_event_rx, watch_event_tx)
)]
pub async fn report(
    watch: Watch,
    mut registrar_event_rx: AddDocumentsEventReceiver,
    mut scanner_event_rx: ScanDirectoryEventReceiver,
    mut watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
) -> Result<()> {
    let mut done = 0;
    let mut failed = 0;
    let mut total = 0;

    loop {
        tokio::select! {
            Some(event) = registrar_event_rx.next() => {
                match event {
                    AddDocumentsEvent::DocumentAdded => {
                        done += 1;
                    }
                    AddDocumentsEvent::AlreadyExists => {
                        // do nothing
                    }
                    AddDocumentsEvent::AddingDocumentFailed => {
                        failed += 1;
                    }
                }
            },
            Some(event) = scanner_event_rx.next() => {
                match event {
                    ScanDirectoryEvent::FileDetected => {
                        total += 1;
                    }
                }
            }
            else => {
                break;
            }
        }

        let report = JobReport {
            watch: watch.clone(),
            progress: JobProgress::new(done, failed, total),
            job_type: JobType::ScanWatchPath,
            status: JobStatus::Running,
        };
        watch_event_tx.send(WatchEvent::AddWatchProgressed { report }).await?;
    }

    watch_event_tx
        .send(WatchEvent::AddWatchFinished {
            watch: watch.active(),
            report: JobReport {
                watch: watch.clone(),
                progress: JobProgress::new(done, failed, total),
                job_type: JobType::ScanWatchPath,
                status: JobStatus::Finished,
            },
        })
        .await?;

    Ok(())
}
