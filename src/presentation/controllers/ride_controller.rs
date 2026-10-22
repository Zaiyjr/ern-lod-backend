use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, patch, post},
    Json, Router,
};
use crate::application::dto::{RequestRideDto, UpdateRideStatusDto};
use crate::presentation::middleware::{map_domain_error, AuthUser};
use crate::presentation::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/request", post(request_ride))
        .route("/pending", get(list_pending_rides))
        .route("/{id}", get(get_ride))
        .route("/{id}/match", post(match_driver))
        .route("/{id}/status", patch(update_ride_status))
}

async fn request_ride(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(payload): Json<RequestRideDto>,
) -> Result<Response, Response> {
    let ride = state
        .ride_usecase
        .request_ride(&auth_user.0.sub, payload)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::CREATED, Json(ride)).into_response())
}

async fn list_pending_rides(
    State(state): State<AppState>,
    _auth_user: AuthUser,
) -> Result<Response, Response> {
    let rides = state
        .ride_usecase
        .list_pending_rides()
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::OK, Json(rides)).into_response())
}

async fn get_ride(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Response, Response> {
    let ride = state
        .ride_usecase
        .get_ride_by_id(&id)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::OK, Json(ride)).into_response())
}

async fn match_driver(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Path(id): Path<String>,
) -> Result<Response, Response> {
    let ride = state
        .ride_usecase
        .match_nearest_driver(&id)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::OK, Json(ride)).into_response())
}

async fn update_ride_status(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateRideStatusDto>,
) -> Result<Response, Response> {
    let ride = state
        .ride_usecase
        .update_status(&id, &auth_user.0.sub, payload)
        .await
        .map_err(map_domain_error)?;

    Ok((StatusCode::OK, Json(ride)).into_response())
}
