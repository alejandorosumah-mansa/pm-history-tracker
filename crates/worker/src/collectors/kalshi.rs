use anyhow::Result;
use chrono::{DateTime, Utc};
use pm_shared::CreateMarket;
use serde::Deserialize;

const KALSHI_API: &str = "https://api.elections.kalshi.com/trade-api/v2";

#[derive(Debug, Deserialize)]
struct KalshiMarketsResponse {
    markets: Vec<KalshiMarket>,
}

#[derive(Debug, Deserialize)]
struct KalshiMarket {
    ticker: String,
    title: String,
    #[serde(default)]
    subtitle: String,
    yes_ask: Option<f64>,
    yes_bid: Option<f64>,
    volume: Option<f64>,
    volume_24h: Option<f64>,
    open_interest: Option<f64>,
    status: String,
    close_time: Option<String>,
    category: Option<String>,
}

pub struct KalshiCollector {
    client: reqwest::Client,
}

impl KalshiCollector {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_markets(&self, limit: usize) -> Result<Vec<CreateMarket>> {
        let url = format!("{}/markets?limit={}&status=open", KALSHI_API, limit);

        tracing::info!("Fetching markets from Kalshi: {}", url);

        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Kalshi API error: {}", response.status());
        }

        let data: KalshiMarketsResponse = response.json().await?;

        tracing::info!("Fetched {} markets from Kalshi", data.markets.len());

        let create_markets = data.markets
            .into_iter()
            .filter_map(|m| self.convert_market(m))
            .collect();

        Ok(create_markets)
    }

    fn convert_market(&self, market: KalshiMarket) -> Option<CreateMarket> {
        // Calculate midpoint price from bid/ask
        let yes_price = match (market.yes_bid, market.yes_ask) {
            (Some(bid), Some(ask)) => ((bid + ask) / 2.0) as f32 / 100.0,
            (Some(bid), None) => (bid as f32) / 100.0,
            (None, Some(ask)) => (ask as f32) / 100.0,
            (None, None) => 0.5,
        };

        let no_price = 1.0 - yes_price;

        let volume = market.volume
            .map(|v| v as f32)
            .unwrap_or(0.0);

        let volume_24h = market.volume_24h
            .map(|v| v as f32)
            .unwrap_or(0.0);

        let liquidity = market.open_interest
            .map(|oi| oi as f32);

        let close_at = market.close_time
            .and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let url = format!("https://kalshi.com/markets/{}", market.ticker);

        Some(CreateMarket {
            source_id: market.ticker.clone(),
            source: "kalshi".to_string(),
            title: market.title,
            description: market.subtitle,
            category: market.category,
            tags: None,
            yes_price,
            no_price,
            volume,
            volume_24h,
            liquidity,
            status: market.status.to_lowercase(),
            close_at,
            url,
        })
    }
}
