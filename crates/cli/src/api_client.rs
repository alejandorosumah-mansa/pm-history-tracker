use anyhow::Result;
use pm_shared::{Market, PriceHistory};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchResultItem>,
}

#[derive(Debug, Deserialize)]
struct SearchResultItem {
    #[serde(flatten)]
    market: Market,
    score: f32,
}

pub struct ApiClient {
    base_url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub async fn search(&self, query: &str, limit: usize) -> Result<Vec<(Market, f32)>> {
        let url = format!("{}/api/search?q={}&limit={}", self.base_url, query, limit);

        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API error: {}", response.status());
        }

        let data: SearchResponse = response.json().await?;

        Ok(data.results.into_iter().map(|r| (r.market, r.score)).collect())
    }

    pub async fn get_market(&self, id: Uuid) -> Result<Market> {
        let url = format!("{}/api/markets/{}", self.base_url, id);

        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API error: {}", response.status());
        }

        Ok(response.json().await?)
    }

    pub async fn get_history(&self, id: Uuid, hours: Option<i64>) -> Result<Vec<PriceHistory>> {
        let mut url = format!("{}/api/markets/{}/history?limit=1000", self.base_url, id);

        if let Some(h) = hours {
            url.push_str(&format!("&hours={}", h));
        }

        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API error: {}", response.status());
        }

        Ok(response.json().await?)
    }

    pub async fn list_markets(&self, limit: usize) -> Result<Vec<Market>> {
        let url = format!("{}/api/markets?limit={}", self.base_url, limit);

        let response = self.client
            .get(&url)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await?;

        if !response.status().is_success() {
            anyhow::bail!("API error: {}", response.status());
        }

        Ok(response.json().await?)
    }
}
