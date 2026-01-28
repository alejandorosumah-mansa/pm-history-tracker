use anyhow::Result;
use chrono::{DateTime, Utc};
use pm_shared::CreateMarket;
use serde::Deserialize;

const POLYMARKET_API: &str = "https://gamma-api.polymarket.com";

#[derive(Debug, Deserialize)]
struct PolymarketMarket {
    #[serde(rename = "conditionId")]
    condition_id: String,
    question: String,
    description: Option<String>,
    #[serde(rename = "outcomePrices")]
    outcome_prices: Vec<String>,
    volume: Option<String>,
    #[serde(rename = "volume24hr")]
    volume_24hr: Option<String>,
    liquidity: Option<String>,
    active: bool,
    #[serde(rename = "endDate")]
    end_date: Option<String>,
    #[serde(rename = "category")]
    category: Option<String>,
    #[serde(rename = "outcomes")]
    outcomes: Vec<String>,
}

pub struct PolymarketCollector {
    client: reqwest::Client,
}

impl PolymarketCollector {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn fetch_markets(&self, limit: usize) -> Result<Vec<CreateMarket>> {
        let url = format!("{}/markets?limit={}&active=true", POLYMARKET_API, limit);

        tracing::info!("Fetching markets from Polymarket: {}", url);

        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(30))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("Polymarket API error: {}", response.status());
        }

        let markets: Vec<PolymarketMarket> = response.json().await?;

        tracing::info!("Fetched {} markets from Polymarket", markets.len());

        let create_markets = markets
            .into_iter()
            .filter_map(|m| self.convert_market(m))
            .collect();

        Ok(create_markets)
    }

    fn convert_market(&self, market: PolymarketMarket) -> Option<CreateMarket> {
        // Parse prices - typically [yes_price, no_price]
        let yes_price = market.outcome_prices.get(0)
            .and_then(|p| p.parse::<f32>().ok())
            .unwrap_or(0.5);

        let no_price = market.outcome_prices.get(1)
            .and_then(|p| p.parse::<f32>().ok())
            .unwrap_or(1.0 - yes_price);

        let volume = market.volume
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0);

        let volume_24h = market.volume_24hr
            .and_then(|v| v.parse::<f32>().ok())
            .unwrap_or(0.0);

        let liquidity = market.liquidity
            .and_then(|l| l.parse::<f32>().ok());

        let close_at = market.end_date
            .and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
            .map(|dt| dt.with_timezone(&Utc));

        let url = format!("https://polymarket.com/event/{}", market.condition_id);

        Some(CreateMarket {
            source_id: market.condition_id,
            source: "polymarket".to_string(),
            title: market.question,
            description: market.description.unwrap_or_default(),
            category: market.category,
            tags: None,
            yes_price,
            no_price,
            volume,
            volume_24h,
            liquidity,
            status: if market.active { "open" } else { "closed" }.to_string(),
            close_at,
            url,
        })
    }
}
