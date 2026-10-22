use axum::{response::Json, routing::get, Router};
use serde_json::{json, Value};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;

use crate::presentation::controllers::{auth_controller, ride_controller};
use crate::presentation::state::AppState;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let api_routes = Router::new()
        .nest("/auth", auth_controller::router())
        .nest("/rides", ride_controller::router());

    Router::new()
        .route("/health", get(health_check))
        .nest("/api/v1", api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "healthy",
        "service": "ride-booking-backend",
        "architecture": "Clean Architecture (Rust)"
    }))
}
