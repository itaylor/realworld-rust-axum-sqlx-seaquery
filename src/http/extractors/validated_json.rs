use axum::{
    Json,
    extract::{FromRequest, Request},
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::de::DeserializeOwned;

use crate::http::dto::error::ErrorResponse;

pub struct ValidatedJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        // Verify content type
        let content_type = req
            .headers()
            .get(header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if !content_type.starts_with("application/json") {
            return Err((
                StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Json(ErrorResponse::field(
                    "body",
                    "content type must be application/json",
                )),
            )
                .into_response());
        }

        let bytes = axum::body::to_bytes(req.into_body(), usize::MAX)
            .await
            .map_err(|_| {
                (
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::field("body", "failed to read request body")),
                )
                    .into_response()
            })?;

        let deserializer = &mut serde_json::Deserializer::from_slice(&bytes);
        match serde_path_to_error::deserialize::<_, T>(deserializer) {
            Ok(value) => Ok(ValidatedJson(value)),
            Err(err) => {
                let path_str = err.path().to_string();
                let inner_str = err.inner().to_string();
                let field = leaf_field(path_str.as_str());
                let message = validation_message(inner_str.as_str());
                Err((
                    StatusCode::UNPROCESSABLE_ENTITY,
                    Json(ErrorResponse::field(field, message)),
                )
                    .into_response())
            }
        }
    }
}

/// Extract the last segment of a dotted path, e.g. "user.username" → "username".
fn leaf_field(path: &str) -> String {
    path.split('.').next_back().unwrap_or(path).to_string()
}

/// Map serde/validation error messages to spec-compliant strings.
fn validation_message(raw: &str) -> String {
    let lower = raw.to_lowercase();

    if lower.contains("blank") || lower.contains("empty") {
        "can't be blank".to_string()
    } else if lower.contains("already taken")
        || lower.contains("already registered")
        || lower.contains("already exists")
    {
        "has already been taken".to_string()
    } else if lower.contains("invalid email") || lower.contains("invalid format") {
        "is invalid".to_string()
    } else if lower.contains("too short") || lower.contains("at least") {
        "is too short".to_string()
    } else if lower.contains("too long") || lower.contains("longer than") {
        "is too long".to_string()
    } else {
        // Strip serde path prefix noise like "invalid value: string \"\" at line 1 column 22: "
        // The useful part is after the last ": "
        if let Some(pos) = raw.rfind(": ") {
            raw[pos + 2..].to_string()
        } else {
            raw.to_string()
        }
    }
}
