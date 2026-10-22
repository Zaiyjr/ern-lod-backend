use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use crate::application::services::Claims;
use crate::domain::errors::DomainError;
use crate::presentation::state::AppState;

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|h| h.to_str().ok());

        let token = match auth_header {
            Some(header) if header.starts_with("Bearer ") => &header[7..],
            _ => {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Missing or invalid Authorization header" })),
                )
                    .into_response())
            }
        };

        match state.token_service.verify_token(token) {
            Ok(claims) => Ok(AuthUser(claims)),
            Err(e) => Err(map_domain_error(e)),
        }
    }
}

pub fn map_domain_error(err: DomainError) -> Response {
    let (status, message) = match err {
        DomainError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        DomainError::ValidationFailed(msg) => (StatusCode::BAD_REQUEST, msg),
        DomainError::Conflict(msg) => (StatusCode::CONFLICT, msg),
        DomainError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
        DomainError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),
        DomainError::InfrastructureError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
    };

    (status, Json(json!({ "error": message }))).into_response()
}
