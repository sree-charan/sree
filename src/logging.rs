use anyhow::Result;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

const MAX_LOG_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const MAX_LOG_FILES: usize = 5;

pub fn init() -> Result<()> {
    let log_dir = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?
        .join(".sree")
        .join("logs");

    std::fs::create_dir_all(&log_dir)?;

    let log_file = log_dir.join("sree.log");
    
    // Rotate logs if needed
    rotate_logs_if_needed(&log_file)?;

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)?;

    let file_layer = fmt::layer()
        .with_writer(std::sync::Arc::new(file))
        .with_ansi(false)
        .with_target(true)
        .with_thread_ids(true);

    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("sree=debug,info"));

    tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .init();

    tracing::info!("Logging initialized");

    Ok(())
}

fn rotate_logs_if_needed(log_file: &std::path::Path) -> Result<()> {
    // Check if log file exists and its size
    if !log_file.exists() {
        return Ok(());
    }

    let metadata = std::fs::metadata(log_file)?;
    if metadata.len() < MAX_LOG_SIZE {
        return Ok(());
    }

    // Rotate existing logs
    let log_dir = log_file.parent().ok_or_else(|| anyhow::anyhow!("Invalid log path"))?;
    
    // Delete oldest log if we have too many
    let oldest_log = log_dir.join(format!("sree.log.{}", MAX_LOG_FILES));
    if oldest_log.exists() {
        std::fs::remove_file(oldest_log)?;
    }

    // Shift all logs: sree.log.N -> sree.log.N+1
    for i in (1..MAX_LOG_FILES).rev() {
        let old_path = log_dir.join(format!("sree.log.{}", i));
        let new_path = log_dir.join(format!("sree.log.{}", i + 1));
        if old_path.exists() {
            std::fs::rename(old_path, new_path)?;
        }
    }

    // Move current log to sree.log.1
    let rotated_path = log_dir.join("sree.log.1");
    std::fs::rename(log_file, rotated_path)?;

    Ok(())
}
