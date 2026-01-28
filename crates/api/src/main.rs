mod config;
mod db;
mod error;
mod routes;

use axum::{
    routing::get,
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use config::Config;
use db::{MarketRepository, PriceHistoryRepository};

// Shared application state
#[derive(Clone)]
struct AppState {
    market_repo: Arc<MarketRepository>,
    history_repo: Arc<PriceHistoryRepository>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pm_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    tracing::info!("Starting PM History Tracker API on {}:{}", config.host, config.port);

    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Connected to database");

    // Create repositories
    let market_repo = Arc::new(MarketRepository::new(pool.clone()));
    let history_repo = Arc::new(PriceHistoryRepository::new(pool.clone()));

    // Create shared app state
    let app_state = AppState {
        market_repo,
        history_repo,
    };

    // Build router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/search", get(routes::search::search_markets))
        .route("/api/markets", get(routes::markets::list_markets))
        .route("/api/markets/:id", get(routes::markets::get_market))
        .route("/api/markets/:id/history", get(routes::history::get_price_history))
        .with_state(app_state)
        .layer(CorsLayer::permissive());

    // Start server
    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn health_check() -> &'static str {
    "OK"
}
