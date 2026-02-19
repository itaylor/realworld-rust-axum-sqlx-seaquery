use crate::http::dto::error::ErrorResponse;
use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0} not found")]
    ResourceNotFound(&'static str),
    #[error("{0} forbidden")]
    ResourceForbidden(&'static str),
    #[error("Token is missing")]
    MissingToken,
    #[error("Token is invalid")]
    InvalidToken,
    #[error("Invalid credentials")]
    InvalidCredentials,
    #[error("Resource not found")]
    NotFound,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Forbidden")]
    Forbidden,
    #[error("Bad request: {0}")]
    BadData(String),
    #[error("Field conflict: {field}: {message}")]
    FieldConflict {
        field: &'static str,
        message: String,
    },
    #[error("Database error")]
    Db(#[from] sqlx::Error),
    #[error("Internal error: {0}")]
    Other(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ResourceNotFound(resource) => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::field(resource, "not found")),
            )
                .into_response(),

            AppError::ResourceForbidden(resource) => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::field(resource, "forbidden")),
            )
                .into_response(),

            AppError::MissingToken => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::field("token", "is missing")),
            )
                .into_response(),

            AppError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::field("token", "is invalid")),
            )
                .into_response(),

            AppError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::field("credentials", "invalid")),
            )
                .into_response(),

            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                Json(ErrorResponse::field("body", "not found")),
            )
                .into_response(),

            AppError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                Json(ErrorResponse::field("body", "unauthorized")),
            )
                .into_response(),

            AppError::Forbidden => (
                StatusCode::FORBIDDEN,
                Json(ErrorResponse::field("body", "forbidden")),
            )
                .into_response(),

            AppError::BadData(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(ErrorResponse::field("body", msg)),
            )
                .into_response(),

            AppError::FieldConflict { field, message } => (
                StatusCode::CONFLICT,
                Json(ErrorResponse::field(field, message)),
            )
                .into_response(),

            AppError::Db(err) => {
                error!("Database error: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::field("body", "database error")),
                )
                    .into_response()
            }

            AppError::Other(err) => {
                error!("Internal: {err:?}");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::field("body", "internal server error")),
                )
                    .into_response()
            }
        }
    }
}
