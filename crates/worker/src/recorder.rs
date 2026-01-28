use anyhow::Result;
use pm_shared::{CreateMarket, Market};
use sqlx::PgPool;

pub struct MarketRecorder {
    pool: PgPool,
}

impl MarketRecorder {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Upsert market and record price history snapshot
    pub async fn record_market(&self, market: CreateMarket) -> Result<Market> {
        // Upsert market
        let updated_market = sqlx::query_as::<_, Market>(
            r#"
            INSERT INTO markets (
                source_id, source, title, description, category, tags,
                yes_price, no_price, volume, volume_24h, liquidity,
                status, close_at, url
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
            ON CONFLICT (source, source_id)
            DO UPDATE SET
                title = EXCLUDED.title,
                description = EXCLUDED.description,
                yes_price = EXCLUDED.yes_price,
                no_price = EXCLUDED.no_price,
                volume = EXCLUDED.volume,
                volume_24h = EXCLUDED.volume_24h,
                liquidity = EXCLUDED.liquidity,
                status = EXCLUDED.status,
                close_at = EXCLUDED.close_at
            RETURNING *
            "#,
        )
        .bind(&market.source_id)
        .bind(&market.source)
        .bind(&market.title)
        .bind(&market.description)
        .bind(&market.category)
        .bind(&market.tags)
        .bind(market.yes_price)
        .bind(market.no_price)
        .bind(market.volume)
        .bind(market.volume_24h)
        .bind(market.liquidity)
        .bind(&market.status)
        .bind(market.close_at)
        .bind(&market.url)
        .fetch_one(&self.pool)
        .await?;

        // Record price history snapshot
        let _ = sqlx::query(
            r#"
            INSERT INTO price_history (
                market_id, yes_price, no_price, volume, volume_24h, liquidity
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (market_id, recorded_at) DO NOTHING
            "#,
        )
        .bind(updated_market.id)
        .bind(updated_market.yes_price)
        .bind(updated_market.no_price)
        .bind(updated_market.volume)
        .bind(updated_market.volume_24h)
        .bind(updated_market.liquidity)
        .execute(&self.pool)
        .await?;

        Ok(updated_market)
    }

    /// Batch record multiple markets
    pub async fn record_markets_batch(&self, markets: Vec<CreateMarket>) -> Result<usize> {
        let mut count = 0;

        for market in markets {
            match self.record_market(market).await {
                Ok(_) => count += 1,
                Err(e) => {
                    tracing::error!("Failed to record market: {}", e);
                }
            }
        }

        Ok(count)
    }
}
