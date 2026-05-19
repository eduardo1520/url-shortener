use axum::{
    Router,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;

use super::handlers::{self, AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::hello))
        .route("/r/:code", get(handlers::redirect))
        .route("/api/shorten", post(handlers::shorten))
        .nest_service("/static", ServeDir::new("static"))
        .layer(CorsLayer::permissive())
        .with_state(state)
}
