use anyhow::Result;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use governor::middleware::NoOpMiddleware;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use thiserror::Error;
use tower_governor::{GovernorLayer, governor::GovernorConfig, key_extractor::SmartIpKeyExtractor};
use tower_http::cors::CorsLayer;
use url::Url;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("URL not found: {0}")]
    NotFound(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error("Internal error")]
    Internal(#[from] sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidUrl(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
            AppError::ServiceUnavailable => (StatusCode::SERVICE_UNAVAILABLE, self.to_string()),
            AppError::Internal(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };
        (status, Json(ErrorResponse { error: message })).into_response()
    }
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Debug, Deserialize)]
struct ShortenRequest {
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ShortenResponse {
    pub id: String,
    pub short_url: String,
    pub original_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub base_url: String,
}

impl AppState {
    fn build_short_url(&self, id: &str) -> String {
        format!("{}/{}", self.base_url, id)
    }
}

// POST /shorten
async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), AppError> {
    let parsed_url =
        Url::parse(&payload.url).map_err(|_| AppError::InvalidUrl(payload.url.clone()))?;

    match parsed_url.scheme() {
        "http" | "https" => {}
        _ => return Err(AppError::InvalidUrl(payload.url)),
    }

    let url = parsed_url.to_string();

    if let Some(row) = sqlx::query!(
        r#"SELECT id, original_url FROM urls WHERE original_url = ?"#,
        url,
    )
    .fetch_optional(&state.db)
    .await?
    {
        tracing::info!(id = %row.id, "Returning existing short URL");
        let short_url = state.build_short_url(&row.id);
        return Ok((
            StatusCode::OK,
            Json(ShortenResponse {
                id: row.id,
                short_url,
                original_url: row.original_url,
            }),
        ));
    }

    let id = loop {
        let id_candidate = nanoid::nanoid!(8);
        let exists = sqlx::query_scalar!(r#"SELECT COUNT(*) FROM urls WHERE id = ?"#, id_candidate)
            .fetch_one(&state.db)
            .await?;
        if exists == 0 {
            break id_candidate;
        }
        tracing::warn!(id_candidate = %id_candidate, "ID collision – retrying");
    };

    sqlx::query!(
        r#"INSERT INTO urls (id, original_url) VALUES (?, ?)"#,
        id,
        url,
    )
    .execute(&state.db)
    .await?;

    let short_url = state.build_short_url(&id);

    tracing::info!(id = %id, url = %url, "URL shortened");

    Ok((
        StatusCode::CREATED,
        Json(ShortenResponse {
            id,
            short_url,
            original_url: url,
        }),
    ))
}

// GET /:id
async fn redirect(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Redirect, AppError> {
    let row = sqlx::query!(r#"SELECT original_url FROM urls WHERE id = ?"#, id)
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| AppError::NotFound(id.clone()))?;

    tracing::info!(id = %id, target = %row.original_url, "Redirecting");
    Ok(Redirect::temporary(&row.original_url))
}

// GET /health
async fn health(State(state): State<AppState>) -> Result<Json<serde_json::Value>, AppError> {
    sqlx::query_scalar::<_, i64>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            tracing::error!(error = %e, "Health check DB ping failed");
            AppError::ServiceUnavailable
        })?;

    Ok(Json(serde_json::json!({ "status": "ok" })))
}

pub fn router(
    state: AppState,
    governor_conf: GovernorConfig<SmartIpKeyExtractor, NoOpMiddleware>,
    cors: CorsLayer,
) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten))
        .route("/{id}", get(redirect))
        .layer(GovernorLayer::new(governor_conf))
        .layer(cors)
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    fn router_test(state: AppState) -> Router {
        Router::new()
            .route("/health", get(health))
            .route("/shorten", post(shorten))
            .route("/{id}", get(redirect))
            .with_state(state)
    }

    async fn test_server() -> TestServer {
        let db = SqlitePool::connect(":memory:").await.unwrap();
        sqlx::migrate!("db/migrations").run(&db).await.unwrap();
        let state = AppState {
            db,
            base_url: "http://localhost:3000".to_string(),
        };
        TestServer::new(router_test(state))
    }

    #[tokio::test]
    async fn health_returns_200() {
        let server = test_server().await;
        server.get("/health").await.assert_status_ok();
    }

    #[tokio::test]
    async fn shorten_valid_url_returns_201() {
        let server = test_server().await;
        let response = server
            .post("/shorten")
            .json(&json!({ "url": "https://example.com" }))
            .await;

        response.assert_status(StatusCode::CREATED);

        let body: ShortenResponse = response.json();
        assert_eq!(body.original_url, "https://example.com/");
        assert!(!body.id.is_empty());
    }

    #[tokio::test]
    async fn shorten_duplicate_url_returns_same_id() {
        let server = test_server().await;

        let first: ShortenResponse = server
            .post("/shorten")
            .json(&json!({ "url": "https://example.com/dup" }))
            .await
            .json();

        let second: ShortenResponse = server
            .post("/shorten")
            .json(&json!({ "url": "https://example.com/dup" }))
            .await
            .json();

        assert_eq!(first.id, second.id);
    }

    #[tokio::test]
    async fn shorten_invalid_url_returns_422() {
        let server = test_server().await;
        server
            .post("/shorten")
            .json(&json!({ "url": "no-url" }))
            .await
            .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn shorten_ftp_returns_422() {
        let server = test_server().await;
        server
            .post("/shorten")
            .json(&json!({ "url": "ftp://example.com" }))
            .await
            .assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn redirect_known_id_returns_307() {
        let server = test_server().await;

        let body: ShortenResponse = server
            .post("/shorten")
            .json(&json!({"url": "https://www.example.com"}))
            .await
            .json();

        server
            .get(&format!("/{}", body.id))
            .await
            .assert_status(StatusCode::TEMPORARY_REDIRECT);
    }

    #[tokio::test]
    async fn redirect_unknown_id_returns_404() {
        let server = test_server().await;
        server
            .get("/nothing-there")
            .await
            .assert_status(StatusCode::NOT_FOUND);
    }
}
