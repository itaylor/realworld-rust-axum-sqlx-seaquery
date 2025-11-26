pub(crate) mod dto;
pub(crate) mod extractors;
pub(crate) mod routes;

use routes::*;

use crate::app_config::AppConfig;
use crate::domain::article_service::ArticleService;
use crate::domain::comment_service::CommentService;
use crate::domain::profile_service::ProfileService;
use crate::domain::tag_service::TagService;
use crate::domain::user_service::UserService;
use crate::openapi::ApiDoc;
use crate::utils::jwt::JwtHandler;
use axum::Router;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn router(state: AppState) -> Router {
    let api_routes = Router::new()
        .merge(auth::auth_routes())
        .merge(users::user_routes())
        .merge(profiles::profile_routes())
        .merge(articles::article_routes())
        .merge(comments::comment_routes())
        .merge(tags::tag_routes())
        .merge(health::health_routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        );

    Router::new()
        .nest("/api", api_routes)
        .merge(
            SwaggerUi::new("/api/docs")
                .url("/api/docs/openapi.json", ApiDoc::openapi())
        )
        .with_state(state)
}

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub user_service: UserService,
    pub article_service: ArticleService,
    pub comment_service: CommentService,
    pub tag_service: TagService,
    pub profile_service: ProfileService,
    pub jwt: JwtHandler,
}
