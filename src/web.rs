use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use tower_http::{cors::CorsLayer, services::ServeDir};

use crate::shortener::{ShortenError, ShortenerService};

pub type AppState = Arc<ShortenerService>;

#[derive(Deserialize)]
struct ShortenRequest {
    url: String,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(hello))
        .route("/r/:code", get(redirect))
        .route("/api/shorten", post(shorten))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}

async fn hello() -> &'static str {
    "Hello, World!"
}

async fn redirect(State(service): State<AppState>, Path(code): Path<String>) -> (StatusCode, HeaderMap) {
    let mut headers = HeaderMap::new();

    match service.resolve(&code) {
        Some(url) => {
            headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
            (StatusCode::FOUND, headers)
        }
        None => (StatusCode::NOT_FOUND, headers),
    }
}

async fn shorten(
    State(service): State<AppState>,
    Json(payload): Json<ShortenRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    match service.shorten(&payload.url) {
        Ok(result) => (
            StatusCode::CREATED,
            Json(serde_json::json!({
                "code": result.code,
                "short_url": format!("/r/{}", result.code)
            })),
        ),
        Err(ShortenError::InvalidUrl) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "URL deve começar com http:// ou https://"
            })),
        ),
    }
}
