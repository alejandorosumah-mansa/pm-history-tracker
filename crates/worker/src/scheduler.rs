use anyhow::Result;
use sqlx::PgPool;
use std::time::Duration;
use tokio::time;

use crate::collectors::{KalshiCollector, PolymarketCollector};
use crate::config::Config;
use crate::recorder::MarketRecorder;

pub struct Scheduler {
    config: Config,
    recorder: MarketRecorder,
    polymarket: PolymarketCollector,
    kalshi: KalshiCollector,
}

impl Scheduler {
    pub fn new(config: Config, pool: PgPool) -> Self {
        Self {
            config,
            recorder: MarketRecorder::new(pool),
            polymarket: PolymarketCollector::new(),
            kalshi: KalshiCollector::new(),
        }
    }

    pub async fn run(&self) -> Result<()> {
        tracing::info!(
            "Starting scheduler with {}s interval, tracking {} markets",
            self.config.collection_interval_seconds,
            self.config.tracked_markets_limit
        );

        let mut interval = time::interval(Duration::from_secs(
            self.config.collection_interval_seconds,
        ));

        loop {
            interval.tick().await;

            tracing::info!("Starting collection cycle");

            if let Err(e) = self.collect_and_record().await {
                tracing::error!("Collection cycle failed: {}", e);
            }
        }
    }

    async fn collect_and_record(&self) -> Result<()> {
        let markets_per_source = self.config.tracked_markets_limit / 2;

        // Collect from Polymarket
        match self.polymarket.fetch_markets(markets_per_source).await {
            Ok(markets) => {
                tracing::info!("Collected {} markets from Polymarket", markets.len());
                match self.recorder.record_markets_batch(markets).await {
                    Ok(count) => tracing::info!("Recorded {} Polymarket markets", count),
                    Err(e) => tracing::error!("Failed to record Polymarket markets: {}", e),
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch Polymarket markets: {}", e);
            }
        }

        // Small delay between sources
        tokio::time::sleep(Duration::from_millis(500)).await;

        // Collect from Kalshi
        match self.kalshi.fetch_markets(markets_per_source).await {
            Ok(markets) => {
                tracing::info!("Collected {} markets from Kalshi", markets.len());
                match self.recorder.record_markets_batch(markets).await {
                    Ok(count) => tracing::info!("Recorded {} Kalshi markets", count),
                    Err(e) => tracing::error!("Failed to record Kalshi markets: {}", e),
                }
            }
            Err(e) => {
                tracing::error!("Failed to fetch Kalshi markets: {}", e);
            }
        }

        tracing::info!("Collection cycle completed");
        Ok(())
    }
}
