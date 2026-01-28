use axum::{
    extract::{Query, State},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::{error::ApiResult, AppState};
use pm_shared::Market;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub source: Option<String>,
    pub status: Option<String>,
}

fn default_limit() -> i64 {
    10
}

#[derive(Debug, Serialize)]
pub struct SearchResponse {
    pub results: Vec<SearchResultItem>,
    pub total: usize,
}

#[derive(Debug, Serialize)]
pub struct SearchResultItem {
    #[serde(flatten)]
    pub market: Market,
    pub score: f32,
}

pub async fn search_markets(
    State(app_state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> ApiResult<Json<SearchResponse>> {
    let limit = params.limit.min(100);

    let results = app_state.market_repo
        .search(
            &params.q,
            limit,
            params.source.as_deref(),
            params.status.as_deref(),
        )
        .await?;

    let total = results.len();
    let items = results
        .into_iter()
        .map(|(market, score)| SearchResultItem { market, score })
        .collect();

    Ok(Json(SearchResponse {
        results: items,
        total,
    }))
}
