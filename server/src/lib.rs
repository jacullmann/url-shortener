use std::sync::Arc;

use url::Url;

use dashmap::DashMap;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Url not found: {0}")]
    NotFound(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound(_) => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::InvalidUrl(_) => (StatusCode::UNPROCESSABLE_ENTITY, self.to_string()),
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ShortenResponse {
    pub id: String,
    pub short_url: String,
    pub original_url: String,
}

#[derive(Clone)]
pub struct AppState {
    pub urls: Arc<DashMap<String, String>>,
    pub base_url: String,
}

impl AppState {
    pub fn new(base_url: String) -> Self {
        Self {
            urls: Arc::new(DashMap::new()),
            base_url,
        }
    }
}
// POST /shorten
async fn shorten(
    State(state): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> Result<(StatusCode, Json<ShortenResponse>), AppError> {
    let parsed = Url::parse(&payload.url).map_err(|_| AppError::InvalidUrl(payload.url.clone()))?;

    match parsed.scheme() {
        "http" | "https" => {}
        _ => return Err(AppError::InvalidUrl(payload.url)),
    }

    let id = nanoid::nanoid!(8);

    let short_url = format!("{}/{id}", state.base_url);

    state.urls.insert(id.clone(), payload.url.clone());

    tracing::info!(id = %id, url = %payload.url, "URL shortened");

    Ok((
        StatusCode::CREATED,
        Json(ShortenResponse {
            short_url,
            original_url: payload.url,
            id,
        }),
    ))
}

// GET :/id
async fn redirect(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Redirect, AppError> {
    let original = state
        .urls
        .get(&id)
        .ok_or_else(|| AppError::NotFound(id.clone()))?
        .clone();

    tracing::info!(id = %id, target = %original, "Redirecting");

    // 307 Redirect
    Ok(Redirect::temporary(&original))
}

// GET /health
async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok"}))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/shorten", post(shorten))
        .route("/{id}", get(redirect))
        .with_state(state)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum_test::TestServer;
    use serde_json::json;

    fn test_server() -> TestServer {
        let state = AppState::new("http://localhost:3000".to_string());
        TestServer::new(router(state))
    }

    #[tokio::test]
    async fn health_returns_200() {
        let server = test_server();
        let response = server.get("/health").await;
        response.assert_status_ok();
    }

    #[tokio::test]
    async fn shorten_valid_url_returns_201() {
        let server = test_server();
        let response = server
            .post("/shorten")
            .json(&json!({ "url": "https://example.com" }))
            .await;

        response.assert_status(StatusCode::CREATED);

        let body: ShortenResponse = response.json();
        assert_eq!(body.original_url, "https://example.com");
        assert!(!body.id.is_empty());
    }

    #[tokio::test]
    async fn shorten_invalid_url_returns_422() {
        let server = test_server();
        let response = server
            .post("/shorten")
            .json(&json!({ "url": "i-am-not-a-url" }))
            .await;

        response.assert_status(StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn redirect_known_id_returns_307() {
        let server = test_server();

        let shorten_response = server
            .post("/shorten")
            .json(&json!({"url": "https://www.example.com"}))
            .await;
        let body: ShortenResponse = shorten_response.json();

        server
            .get(&format!("/{}", body.id))
            .await
            .assert_status(StatusCode::TEMPORARY_REDIRECT);
    }

    #[tokio::test]
    async fn redirect_unknown_id_returns_404() {
        let server = test_server();
        let response = server.get("/nothing-there").await;
        response.assert_status(StatusCode::NOT_FOUND);
    }
}
