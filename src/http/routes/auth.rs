use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::login::LoginRequest;
use crate::http::dto::register::RegisterRequest;
use crate::http::dto::user::{UserData, UserResponse};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::post;
use axum::{Json, Router};
use tracing::{error, info};

pub(crate) fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/users/login", post(login))
        .route("/users", post(register))
}

async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, StatusCode> {
    info!("Login attempt for email: {}", payload.user.email);

    // TODO: Validate credentials against database
    let user = UserData {
        email: payload.user.email,
        token: "mock.jwt.token".to_string(),
        username: "mockuser".try_into().unwrap(),
        bio: Some("I am a mock user".to_string()),
        image: None,
    };

    Ok(Json(UserResponse { user }))
}

async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!("Registration attempt for email: {}", payload.user.email);

    let user = app_state.user_service.register_user(payload.into()).await?;

    let user = UserData {
        email: user.email,
        token: "mock.jwt.token".to_string(),
        username: user.username,
        bio: None,
        image: None,
    };

    Ok(Json(UserResponse { user }))
}
