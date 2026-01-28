use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Prediction market metadata and current state
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Market {
    pub id: Uuid,
    pub source_id: String,
    pub source: String,
    pub title: String,
    pub description: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub yes_price: f32,
    pub no_price: f32,
    pub volume: f32,
    pub volume_24h: f32,
    pub liquidity: Option<f32>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub close_at: Option<DateTime<Utc>>,
    pub url: String,
}

/// Time-series snapshot of market prices and metrics
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PriceHistory {
    pub id: Uuid,
    pub market_id: Uuid,
    pub yes_price: f32,
    pub no_price: f32,
    pub volume: f32,
    pub volume_24h: f32,
    pub liquidity: Option<f32>,
    pub recorded_at: DateTime<Utc>,
}

/// Request to create a new market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMarket {
    pub source_id: String,
    pub source: String,
    pub title: String,
    pub description: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub yes_price: f32,
    pub no_price: f32,
    pub volume: f32,
    pub volume_24h: f32,
    pub liquidity: Option<f32>,
    pub status: String,
    pub close_at: Option<DateTime<Utc>>,
    pub url: String,
}

/// Request to update a market
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMarket {
    pub yes_price: Option<f32>,
    pub no_price: Option<f32>,
    pub volume: Option<f32>,
    pub volume_24h: Option<f32>,
    pub liquidity: Option<f32>,
    pub status: Option<String>,
    pub close_at: Option<DateTime<Utc>>,
}

/// Search result with relevance score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResult {
    #[serde(flatten)]
    pub market: Market,
    pub score: i64,
}

/// Market source platforms
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MarketSource {
    Polymarket,
    Kalshi,
}

impl MarketSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarketSource::Polymarket => "polymarket",
            MarketSource::Kalshi => "kalshi",
        }
    }
}

impl std::fmt::Display for MarketSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::str::FromStr for MarketSource {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "polymarket" => Ok(MarketSource::Polymarket),
            "kalshi" => Ok(MarketSource::Kalshi),
            _ => Err(format!("Unknown market source: {}", s)),
        }
    }
}

/// Market status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MarketStatus {
    Open,
    Closed,
    Resolved,
}

impl MarketStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            MarketStatus::Open => "open",
            MarketStatus::Closed => "closed",
            MarketStatus::Resolved => "resolved",
        }
    }
}

impl std::fmt::Display for MarketStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
