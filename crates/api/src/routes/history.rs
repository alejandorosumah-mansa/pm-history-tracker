use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::ApiResult, AppState};
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
    State(app_state): State<AppState>,
    Path(market_id): Path<Uuid>,
    Query(params): Query<HistoryQuery>,
) -> ApiResult<Json<Vec<PriceHistory>>> {
    let limit = params.limit.min(1000);
    let history = app_state.history_repo.get_history(market_id, limit, params.hours).await?;
    Ok(Json(history))
}
