use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::sync::Arc;

use crate::api::auth::Claims;

#[derive(Clone)]
pub struct AuthConfig {
    pub enabled: bool,
    pub token: Arc<String>,
}

impl AuthConfig {
    pub fn new(enabled: bool, token: String) -> Self {
        Self {
            enabled,
            token: Arc::new(token),
        }
    }
}

use crate::state::AppState;

pub async fn auth_middleware(
    axum::extract::State(app_state): axum::extract::State<AppState>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth if disabled
    if !app_state.auth_config.enabled {
        return Ok(next.run(request).await);
    }

    let auth_header = request
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    if auth_header.starts_with("Bearer ") {
        let token = &auth_header[7..];

        // 1. Check admin/shared token (backwards compat)
        if token == app_state.auth_config.token.as_str() {
            return Ok(next.run(request).await);
        }

        // 2. Try JWT validation
        let validation = Validation::default();
        if decode::<Claims>(
            token,
            &DecodingKey::from_secret(app_state.jwt_secret.as_bytes()),
            &validation,
        )
        .is_ok()
        {
            return Ok(next.run(request).await);
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}
