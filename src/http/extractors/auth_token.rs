use crate::app_error::AppError;
use crate::http::AppState;
use crate::model::values::user_id::UserId;
use axum::{
    extract::FromRequestParts,
    http::request::Parts,
    response::{IntoResponse, Response},
};
use uuid::Uuid;

pub struct AuthToken {
    pub(crate) user_id: UserId,
    pub(crate) raw_token: String,
}

impl FromRequestParts<AppState> for Option<AuthToken> {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let jwt = &state.jwt;

        let maybe_raw_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        if let Some(raw_header) = maybe_raw_header {
            let token = raw_header
                .strip_prefix("Token ")
                .ok_or_else(|| AppError::InvalidToken.into_response())?;

            let parsed_token = jwt
                .verify_token(token)
                .map_err(|_| AppError::InvalidToken.into_response())?;

            let uuid: Uuid = parsed_token
                .sub
                .parse()
                .map_err(|_| AppError::InvalidToken.into_response())?;

            let user_id = UserId::from(uuid);

            Ok(Some(AuthToken {
                user_id,
                raw_token: token.to_string(),
            }))
        } else {
            Ok(None)
        }
    }
}

impl FromRequestParts<AppState> for AuthToken {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let maybe_token = Option::<AuthToken>::from_request_parts(parts, state).await?;
        maybe_token.ok_or_else(|| AppError::MissingToken.into_response())
    }
}
