use crate::state::AppState;
use axum::{extract::State, http::StatusCode, Json};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct AuthRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,    // user id (UUID)
    pub username: String,
    pub exp: u64,       // unix timestamp
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, &'static str)> {
    let username = req.username.trim().to_string();

    if username.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Username required"));
    }
    if req.password.len() < 6 {
        return Err((StatusCode::BAD_REQUEST, "Password must be at least 6 characters"));
    }
    if state.users.find_by_username(&username).is_some() {
        return Err((StatusCode::CONFLICT, "Username already taken"));
    }

    // bcrypt is CPU-intensive — run on a blocking thread
    let password = req.password.clone();
    let password_hash = tokio::task::spawn_blocking(move || hash(password, DEFAULT_COST))
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hash error"))?
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Hash error"))?;

    let user = state
        .users
        .create(username.clone(), password_hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Storage error"))?;

    let token = make_jwt(&user.id.to_string(), &user.username, &state.jwt_secret)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token error"))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
    }))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<AuthRequest>,
) -> Result<Json<AuthResponse>, (StatusCode, &'static str)> {
    let username = req.username.trim().to_string();

    let user = state
        .users
        .find_by_username(&username)
        .ok_or((StatusCode::UNAUTHORIZED, "Invalid username or password"))?;

    let hash = user.password_hash.clone();
    let password = req.password.clone();
    let ok = tokio::task::spawn_blocking(move || verify(password, &hash))
        .await
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Verify error"))?
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Verify error"))?;

    if !ok {
        return Err((StatusCode::UNAUTHORIZED, "Invalid username or password"));
    }

    let token = make_jwt(&user.id.to_string(), &user.username, &state.jwt_secret)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Token error"))?;

    Ok(Json(AuthResponse {
        token,
        username: user.username,
    }))
}

/// Returns whether any users have been registered.
/// Used by the frontend to decide whether to show Login or Register first.
pub async fn auth_status(
    State(state): State<AppState>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "has_users": state.users.count() > 0 }))
}

pub fn make_jwt(
    user_id: &str,
    username: &str,
    secret: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let exp = (Utc::now().timestamp() as u64) + 30 * 24 * 3600; // 30 days
    let claims = Claims {
        sub: user_id.to_string(),
        username: username.to_string(),
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}
