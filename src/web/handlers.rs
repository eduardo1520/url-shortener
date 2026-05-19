use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::Json,
};

use crate::application::{ShortenError, ShortenerService};
use crate::infrastructure::MemoryLinkRepository;

use super::dto::ShortenRequest;

pub type AppState = Arc<ShortenerService<MemoryLinkRepository>>;

pub async fn hello() -> &'static str {
    "Hello, World!"
}

pub async fn redirect(
    State(service): State<AppState>,
    Path(code): Path<String>,
) -> (StatusCode, HeaderMap) {
    let mut headers = HeaderMap::new();

    match service.resolve(&code) {
        Some(url) => {
            headers.insert(axum::http::header::LOCATION, url.parse().unwrap());
            (StatusCode::FOUND, headers)
        }
        None => (StatusCode::NOT_FOUND, headers),
    }
}

pub async fn shorten(
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
