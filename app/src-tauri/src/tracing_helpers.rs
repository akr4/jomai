use std::path::Path;

use anyhow::Result;
use tracing_subscriber::fmt;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

const LOG_FILE_NAME: &str = "main.log";

pub fn init<P: AsRef<Path>>(_log_dir: P) -> Result<()> {
    let stdout = fmt::layer();

    // let log_file = log_dir.as_ref().join(LOG_FILE_NAME);
    //
    // // for new tracing-appender version
    // // let file_appender = RollingFileAppender::builder()
    // //     .rotation(Rotation::MINUTELY)
    // //     .filename_prefix("main.")
    // //     .filename_suffix(".log")
    // //     .build(log_dir)?;
    // let file_appender = RollingFileAppender::new(Rotation::NEVER, log_dir, LOG_FILE_NAME);
    // let file = fmt::layer().with_ansi(false).with_writer(file_appender);

    let filter = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    tracing_subscriber::registry()
        .with(filter)
        .with(stdout)
        // .with(file)
        .try_init()?;

    Ok(())
}
