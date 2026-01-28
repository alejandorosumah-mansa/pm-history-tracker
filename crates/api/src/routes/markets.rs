use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{error::ApiResult, AppState};
use pm_shared::Market;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default = "default_order")]
    pub order: String,
}

fn default_limit() -> i64 {
    20
}

fn default_sort() -> String {
    "created_at".to_string()
}

fn default_order() -> String {
    "desc".to_string()
}

pub async fn list_markets(
    State(app_state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> ApiResult<Json<Vec<Market>>> {
    let limit = params.limit.min(100);
    let markets = app_state.market_repo.list(limit, params.offset, &params.sort, &params.order).await?;
    Ok(Json(markets))
}

pub async fn get_market(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Market>> {
    let market = app_state.market_repo.get_by_id(id).await?;
    Ok(Json(market))
}
