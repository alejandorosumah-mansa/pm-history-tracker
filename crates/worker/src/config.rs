use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub collection_interval_seconds: u64,
    pub tracked_markets_limit: usize,
    pub enabled: bool,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        let collection_interval_seconds = env::var("COLLECTION_INTERVAL_SECONDS")
            .unwrap_or_else(|_| "3600".to_string())
            .parse()?;

        let tracked_markets_limit = env::var("TRACKED_MARKETS")
            .unwrap_or_else(|_| "10".to_string())
            .parse()?;

        let enabled = env::var("WORKER_ENABLED")
            .unwrap_or_else(|_| "true".to_string())
            .parse()?;

        Ok(Config {
            database_url,
            collection_interval_seconds,
            tracked_markets_limit,
            enabled,
        })
    }
}
