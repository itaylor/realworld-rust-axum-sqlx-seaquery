use crate::app_error::AppError;
use crate::http::AppState;
use crate::http::dto::login::LoginRequest;
use crate::http::dto::register::RegisterRequest;
use crate::http::dto::user::{UserData, UserResponse};
use axum::extract::State;
use axum::routing::post;
use axum::{Json, Router};
use tracing::info;
use crate::domain::commands::register_command::RegisterCommand;

pub(crate) fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/users/login", post(login))
        .route("/users", post(register))
}

async fn login(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!("Login attempt for email: {}", payload.user.email);

    let user = app_state
        .user_service
        .login_user(payload.user.email, payload.user.password)
        .await?;

    let token = app_state.jwt.generate_token(user.id)?;

    let user = UserData::new(user, token);

    Ok(Json(UserResponse { user }))
}

async fn register(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, AppError> {
    info!("Registration attempt for email: {}", payload.user.email);

    let command = RegisterCommand::new(payload);

    let user = app_state.user_service.register_user(command).await?;

    let token = app_state.jwt.generate_token(user.id)?;

    let user = UserData {
        email: user.email,
        token,
        username: user.username,
        bio: user.bio,
        image: user.image,
    };

    Ok(Json(UserResponse { user }))
}
