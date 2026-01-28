use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

use crate::{db::PriceHistoryRepository, error::ApiResult};
use pm_shared::PriceHistory;

#[derive(Debug, Deserialize)]
pub struct HistoryQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    pub hours: Option<i64>,
}

fn default_limit() -> i64 {
    100
}

pub async fn get_price_history(
    State(repo): State<Arc<PriceHistoryRepository>>,
    Path(market_id): Path<Uuid>,
    Query(params): Query<HistoryQuery>,
) -> ApiResult<Json<Vec<PriceHistory>>> {
    let limit = params.limit.min(1000);
    let history = repo.get_history(market_id, limit, params.hours).await?;
    Ok(Json(history))
}
