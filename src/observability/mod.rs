use crate::configuration::Config;
use crate::errors::Result;
use tracing_subscriber::{EnvFilter, fmt, filter::LevelFilter, FmtSubscriber};

pub fn init_logging(config: &Config) -> Result<()> {
    let log_file = std::fs::File::create(config.get_log_path())?;

    // Set up tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive(LevelFilter::INFO.into()))
        .with_writer(std::sync::Mutex::new(log_file))
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(|e| {
        crate::errors::AppError::ConfigError(format!("Failed to init logging: {}", e))
    })?;

    tracing::info!("Logging initialized. Log file: {:?}", config.get_log_path());
    Ok(())
}
