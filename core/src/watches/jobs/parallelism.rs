pub(super) fn available_parallelism() -> usize {
    match std::thread::available_parallelism() {
        Ok(parallelism) => {
            tracing::debug!("available parallelism: {}", parallelism.get());
            parallelism.get()
        }
        Err(e) => {
            tracing::warn!("Failed to get available parallelism, using 1. reason={}", e);
            1
        }
    }
}
