use crate::http::AppState;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};

pub(crate) fn health_routes() -> Router<AppState> {
    Router::new().route("/health", get(health))
}

async fn health() -> impl IntoResponse {
    Json::from("OK").into_response()
}
