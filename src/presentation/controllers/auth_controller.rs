use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use crate::application::dto::{LoginDto, RegisterDto};
use crate::presentation::middleware::{map_domain_error, AuthUser};
use crate::presentation::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(get_current_user))
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterDto>,
) -> Result<Response, Response> {
    let response = state
        .auth_usecase
        .register(payload)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::CREATED, Json(response)).into_response())
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginDto>,
) -> Result<Response, Response> {
    let response = state
        .auth_usecase
        .login(payload)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::OK, Json(response)).into_response())
}

async fn get_current_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Response, Response> {
    let user = state
        .auth_usecase
        .get_user_profile(&auth_user.0.sub)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::OK, Json(user)).into_response())
}
