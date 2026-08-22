mod app;
mod commands;
mod configuration;
mod domain;
mod errors;
mod events;
mod infrastructure;
mod observability;
mod ui;

use app::App;
use configuration::Config;
use errors::Result;
use events::EventBus;
use infrastructure::{platform::PlatformManager, storage::database::Database};

#[tokio::main]
async fn main() -> Result<()> {
    // 1. Load configuration
    let config = Config::load()?;

    // 2. Setup observability (logging)
    observability::init_logging(&config)?;
    tracing::info!("Starting NovaTask - Phase 0");

    // 3. Initialize Database
    let database = Database::init(&config).await?;

    // 4. Initialize Platform Manager
    let platform = PlatformManager::new()?;

    // 5. Setup Event Bus
    let event_bus = EventBus::new(100);

    // 6. Create and run App
    let mut app = App::new(config, database, platform, event_bus).await;

    // We catch errors to ensure terminal is restored if something panics or fails outside the TUI loop
    if let Err(e) = app.run().await {
        tracing::error!("Application error: {}", e);
        eprintln!("Error: {}", e);
    }

    tracing::info!("NovaTask stopped successfully");
    Ok(())
}
