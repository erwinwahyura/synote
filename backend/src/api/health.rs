use axum::{
    extract::State,
    response::{IntoResponse, Json},
    http::StatusCode,
};
use serde::Serialize;
use crate::state::AppState;
use chrono::Utc;

#[derive(Serialize)]
pub struct HealthResponse {
    status: String,
    version: &'static str,
    timestamp: String,
    notes_count: usize,
}

pub async fn health_check(
    State(app_state): State<AppState>,
) -> Result<Json<HealthResponse>, StatusCode> {
    // Try to list notes to verify storage is working
    let notes_count = app_state.storage.list()
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?
        .len();

    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION"),
        timestamp: Utc::now().to_rfc3339(),
        notes_count,
    }))
}

pub async fn readiness_check() -> impl IntoResponse {
    StatusCode::OK
}
