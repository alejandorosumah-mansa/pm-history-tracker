use pm_shared::{Market, PriceHistory, CreateMarket, UpdateMarket};
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct MarketRepository {
    pool: PgPool,
}

impl MarketRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, market: CreateMarket) -> Result<Market, sqlx::Error> {
        let result = sqlx::query_as::<_, Market>(
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

        Ok(result)
    }

    pub async fn get_by_id(&self, id: Uuid) -> Result<Market, sqlx::Error> {
        sqlx::query_as::<_, Market>("SELECT * FROM markets WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
    }

    pub async fn list(
        &self,
        limit: i64,
        offset: i64,
        sort_by: &str,
        order: &str,
    ) -> Result<Vec<Market>, sqlx::Error> {
        let order_clause = match order.to_lowercase().as_str() {
            "asc" => "ASC",
            _ => "DESC",
        };

        let sort_column = match sort_by {
            "volume" => "volume",
            "created_at" => "created_at",
            "close_at" => "close_at",
            "volume_24h" => "volume_24h",
            _ => "created_at",
        };

        let query = format!(
            "SELECT * FROM markets ORDER BY {} {} NULLS LAST LIMIT $1 OFFSET $2",
            sort_column, order_clause
        );

        sqlx::query_as::<_, Market>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await
    }

    pub async fn search(
        &self,
        query: &str,
        limit: i64,
        source: Option<&str>,
        status: Option<&str>,
    ) -> Result<Vec<(Market, f32)>, sqlx::Error> {
        let mut sql = String::from(
            r#"
            SELECT
                m.*,
                ts_rank(to_tsvector('english', m.title || ' ' || m.description),
                        plainto_tsquery('english', $1)) as rank
            FROM markets m
            WHERE to_tsvector('english', m.title || ' ' || m.description) @@ plainto_tsquery('english', $1)
            "#,
        );

        if source.is_some() {
            sql.push_str(" AND m.source = $2");
        }

        if status.is_some() {
            sql.push_str(if source.is_some() {
                " AND m.status = $3"
            } else {
                " AND m.status = $2"
            });
        }

        sql.push_str(" ORDER BY rank DESC, m.volume DESC LIMIT ");
        sql.push_str(&(limit + 2).to_string());

        let mut query_builder = sqlx::query(&sql).bind(query);

        if let Some(src) = source {
            query_builder = query_builder.bind(src);
        }

        if let Some(st) = status {
            query_builder = query_builder.bind(st);
        }

        let rows = query_builder.fetch_all(&self.pool).await?;

        let results = rows
            .into_iter()
            .map(|row| {
                let market = Market {
                    id: row.get("id"),
                    source_id: row.get("source_id"),
                    source: row.get("source"),
                    title: row.get("title"),
                    description: row.get("description"),
                    category: row.get("category"),
                    tags: row.get("tags"),
                    yes_price: row.get("yes_price"),
                    no_price: row.get("no_price"),
                    volume: row.get("volume"),
                    volume_24h: row.get("volume_24h"),
                    liquidity: row.get("liquidity"),
                    status: row.get("status"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                    close_at: row.get("close_at"),
                    url: row.get("url"),
                };
                let rank: f32 = row.get("rank");
                (market, rank)
            })
            .collect();

        Ok(results)
    }

    pub async fn update(&self, id: Uuid, update: UpdateMarket) -> Result<Market, sqlx::Error> {
        let market = self.get_by_id(id).await?;

        let result = sqlx::query_as::<_, Market>(
            r#"
            UPDATE markets
            SET
                yes_price = COALESCE($2, yes_price),
                no_price = COALESCE($3, no_price),
                volume = COALESCE($4, volume),
                volume_24h = COALESCE($5, volume_24h),
                liquidity = COALESCE($6, liquidity),
                status = COALESCE($7, status),
                close_at = COALESCE($8, close_at)
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(update.yes_price)
        .bind(update.no_price)
        .bind(update.volume)
        .bind(update.volume_24h)
        .bind(update.liquidity)
        .bind(update.status)
        .bind(update.close_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }
}

pub struct PriceHistoryRepository {
    pool: PgPool,
}

impl PriceHistoryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn record_snapshot(&self, market_id: Uuid, market: &Market) -> Result<PriceHistory, sqlx::Error> {
        let result = sqlx::query_as::<_, PriceHistory>(
            r#"
            INSERT INTO price_history (
                market_id, yes_price, no_price, volume, volume_24h, liquidity
            )
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (market_id, recorded_at) DO NOTHING
            RETURNING *
            "#,
        )
        .bind(market_id)
        .bind(market.yes_price)
        .bind(market.no_price)
        .bind(market.volume)
        .bind(market.volume_24h)
        .bind(market.liquidity)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn get_history(
        &self,
        market_id: Uuid,
        limit: i64,
        hours: Option<i64>,
    ) -> Result<Vec<PriceHistory>, sqlx::Error> {
        let query = if let Some(h) = hours {
            sqlx::query_as::<_, PriceHistory>(
                r#"
                SELECT * FROM price_history
                WHERE market_id = $1
                  AND recorded_at >= NOW() - INTERVAL '1 hour' * $2
                ORDER BY recorded_at DESC
                LIMIT $3
                "#,
            )
            .bind(market_id)
            .bind(h)
            .bind(limit)
        } else {
            sqlx::query_as::<_, PriceHistory>(
                r#"
                SELECT * FROM price_history
                WHERE market_id = $1
                ORDER BY recorded_at DESC
                LIMIT $2
                "#,
            )
            .bind(market_id)
            .bind(limit)
        };

        query.fetch_all(&self.pool).await
    }
}
