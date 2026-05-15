use anyhow::{Context, Result};
use server::{AppState, router};

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "url_shortener=info".into()),
        )
        .init();

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());

    let base_url = std::env::var("BASE_URL").unwrap_or_else(|_| format!("http://localhost:{port}"));

    let state = AppState::new(base_url);

    let app = router(state);

    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .context("Failed to bind port 3000")?;

    tracing::info!("Listening on http://{addr}");

    axum::serve(listener, app).await.context("Server error")?;

    Ok(())
}
