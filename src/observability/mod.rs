use crate::configuration::Config;
use crate::errors::Result;
use tracing_subscriber::{EnvFilter, fmt};

pub fn init_logging(config: &Config) -> Result<()> {
    let log_file = std::fs::File::create(&config.log_path)?;

    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with_writer(std::sync::Arc::new(log_file))
        .finish();

    tracing::subscriber::set_global_default(subscriber).map_err(|e| {
        crate::errors::AppError::ConfigError(format!("Failed to init logging: {}", e))
    })?;

    tracing::info!("Logging initialized. Log file: {:?}", config.log_path);
    Ok(())
}
