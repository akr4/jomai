use anyhow::Result;
use futures::{channel::mpsc, SinkExt, StreamExt};
use tracing::instrument;

use crate::{
    watches::{
        jobs::{
            tasks::{
                add_documents::AddDocumentsEventReceiver, get_documents::GetDocumentsEvent,
                scan_directory::ScanDirectoryEventReceiver, update_documents::UpdateDocumentsEvent,
            },
            JobProgress, JobStatus, JobType,
        },
        WatchEvent,
    },
    JobReport, Watch,
};

use super::super::tasks::{add_documents::AddDocumentsEvent, scan_directory::ScanDirectoryEvent};

#[instrument(
    name = "sync_watch_job::report",
    level = "info",
    skip(
        get_documents_event_rx,
        update_documents_event_rx,
        add_documents_event_rx,
        scan_directory_event_rx,
        watch_event_tx
    )
)]
pub async fn report(
    watch: Watch,
    mut get_documents_event_rx: mpsc::UnboundedReceiver<GetDocumentsEvent>,
    mut update_documents_event_rx: mpsc::UnboundedReceiver<UpdateDocumentsEvent>,
    mut add_documents_event_rx: AddDocumentsEventReceiver,
    mut scan_directory_event_rx: ScanDirectoryEventReceiver,
    mut watch_event_tx: mpsc::UnboundedSender<WatchEvent>,
) -> Result<()> {
    let mut done = 0;
    let mut failed = 0;
    let mut total = 0;

    loop {
        tokio::select! {
            Some(event) = get_documents_event_rx.next() => {
                match event {
                    GetDocumentsEvent::DocumentFound => {
                        total += 1;
                    }
                }
            },
            Some(event) = update_documents_event_rx.next() => {
                match event {
                    UpdateDocumentsEvent::UpToDate => {
                        done += 1;
                    }
                    UpdateDocumentsEvent::DocumentDeleted => {
                        done += 1;
                    }
                    UpdateDocumentsEvent::DeletingDocumentFailed => {
                        failed += 1;
                    }
                    UpdateDocumentsEvent::DocumentUpdated => {
                        done += 1;
                    }
                    UpdateDocumentsEvent::UpdatingDocumentFailed => {
                        failed += 1;
                    }
                }
            },
            Some(event) = add_documents_event_rx.next() => {
                match event {
                    AddDocumentsEvent::DocumentAdded => {
                        total += 1;
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
            Some(event) = scan_directory_event_rx.next() => {
                match event {
                    ScanDirectoryEvent::FileDetected => {
                        // do nothing
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
            job_type: JobType::SyncWatch,
            status: JobStatus::Running,
        };
        watch_event_tx.send(WatchEvent::SyncWatchProgressed { report }).await?;
    }

    watch_event_tx
        .send(WatchEvent::SyncWatchFinished {
            watch: watch.active(),
            report: JobReport {
                watch: watch.clone(),
                progress: JobProgress::new(done, failed, total),
                job_type: JobType::SyncWatch,
                status: JobStatus::Finished,
            },
        })
        .await?;

    Ok(())
}
