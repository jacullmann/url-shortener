use anyhow::{Context, Result};
use axum::http::{HeaderValue, Method, header};
use server::{AppState, router};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions};
use std::net::SocketAddr;
use std::str::FromStr;
use tower_governor::{governor::GovernorConfigBuilder, key_extractor::SmartIpKeyExtractor};
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=info".into()),
        )
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{port}"));

    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite:urls.db".to_string());

    let rate_limit_per_second = std::env::var("RATE_LIMIT_PER_SECOND")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(1);

    let rate_limit_burst = std::env::var("RATE_LIMIT_BURST")
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(60);

    let cors = build_cors_layer();

    let connect_options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true)
        .journal_mode(SqliteJournalMode::Wal);

    let db = SqlitePoolOptions::new()
        .max_connections(16)
        .connect_with(connect_options)
        .await
        .context("Failed to connect to SQLite")?;

    sqlx::migrate!("db/migrations")
        .run(&db)
        .await
        .context("Failed to run migrations")?;

    let state = AppState { db, base_url };

    let governor_conf = GovernorConfigBuilder::default()
        .key_extractor(SmartIpKeyExtractor)
        .per_second(rate_limit_per_second)
        .burst_size(rate_limit_burst)
        .finish()
        .expect("Invalid governor config");

    let app = router(state, governor_conf, cors);

    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind port")?;

    tracing::info!("Listening on http://{addr}");

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .context("Server error")?;

    Ok(())
}

fn build_cors_layer() -> CorsLayer {
    let origin = std::env::var("CORS_ORIGIN").unwrap_or_default();

    if origin.is_empty() {
        tracing::warn!("CORS_ORIGIN not set – CORS disabled");
        return CorsLayer::new();
    }

    match origin.parse::<HeaderValue>() {
        Ok(value) => {
            tracing::info!(origin = %origin, "CORS configured");
            CorsLayer::new()
                .allow_origin(value)
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([header::CONTENT_TYPE])
        }
        Err(_) => {
            tracing::error!(origin = %origin, "Invalid CORS_ORIGIN – CORS disabled");
            CorsLayer::new()
        }
    }
}
