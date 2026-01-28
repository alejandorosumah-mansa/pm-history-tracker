mod collectors;
mod config;
mod recorder;
mod scheduler;

use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use scheduler::Scheduler;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pm_worker=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;

    if !config.enabled {
        tracing::info!("Worker is disabled via WORKER_ENABLED=false");
        return Ok(());
    }

    tracing::info!("Starting PM History Tracker Worker");
    tracing::info!("Collection interval: {}s", config.collection_interval_seconds);
    tracing::info!("Tracked markets limit: {}", config.tracked_markets_limit);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Connected to database");

    // Create and run scheduler
    let scheduler = Scheduler::new(config, pool);
    scheduler.run().await?;

    Ok(())
}
