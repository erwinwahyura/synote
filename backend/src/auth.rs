use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

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

    // Check for Authorization header
    let headers = request.headers();
    if let Some(auth_header) = headers.get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            // Check Bearer token
            if auth_str.starts_with("Bearer ") {
                let token = &auth_str[7..];
                if token == app_state.auth_config.token.as_str() {
                    return Ok(next.run(request).await);
                }
            }
        }
    }

    // Also check for token in query param (for initial frontend load)
    // This is a convenience for single-user setups
    let uri = request.uri().to_string();
    if uri.contains("token=") {
        if let Some(token_start) = uri.find("token=") {
            let token_part = &uri[token_start + 6..];
            let token = token_part.split('&').next().unwrap_or(token_part);
            if token == app_state.auth_config.token.as_str() {
                return Ok(next.run(request).await);
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_auth_disabled() {
        let config = AuthConfig::new(false, "test-token".to_string());
        // Would need to set up a test route to fully test
        assert!(!config.enabled);
    }

    #[tokio::test]
    async fn test_auth_enabled() {
        let config = AuthConfig::new(true, "secret-token".to_string());
        assert!(config.enabled);
        assert_eq!(config.token.as_str(), "secret-token");
    }
}
