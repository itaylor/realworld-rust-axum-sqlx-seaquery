use crate::http::AppState;
use crate::http::dto::tag::TagsResponse;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Json, Router};
use tracing::info;

pub(crate) fn tag_routes() -> Router<AppState> {
    Router::new().route("/tags", get(get_tags))
}

async fn get_tags() -> impl IntoResponse {
    info!("Get tags");

    // TODO: Fetch tags from database
    let tags = vec!["mock".to_string(), "test".to_string(), "demo".to_string()];

    Json(TagsResponse { tags }).into_response()
}
