mod api;
mod auth;
mod config;
mod links;
mod models;
// mod search; // Disabled - Tantivy removed temporarily
mod state;
mod storage;
// mod sync; // Disabled - git2 compilation errors
mod tags;
mod users;

use crate::api::auth::{auth_status, login, register};
use crate::api::graph::get_graph;
use crate::api::health::{health_check, readiness_check};
use crate::api::links::get_note_links;
use crate::api::notes::{create_note, delete_note, get_note, list_notes, search_notes, update_note};
use crate::api::tags::{get_note_tags, get_tagged_notes, list_tags};
use crate::auth::{auth_middleware, AuthConfig};
use crate::config::Config;
use crate::links::LinksIndex;
use crate::state::AppState;
use crate::storage::NoteStorage;
use crate::tags::TagIndex;
use crate::users::UserStorage;
use axum::{
    middleware,
    routing::{delete, get, post, put},
    Router,
};
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "synote=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::load()?;
    tracing::info!("Configuration loaded: {:?}", config);

    let notes_dir = config.storage.notes_dir.clone();
    let storage = Arc::new(NoteStorage::new(notes_dir.clone())?);
    tracing::info!("Note storage initialized");

    // User storage lives next to notes (data/users.json)
    let data_dir = notes_dir.parent().unwrap_or(&notes_dir).to_path_buf();
    let users = Arc::new(UserStorage::new(&data_dir)?);
    tracing::info!("User storage initialized ({} users)", users.count());

    let links_index = Arc::new(LinksIndex::new());
    let tags_index = Arc::new(TagIndex::new());

    let auth_config = AuthConfig::new(config.auth.enabled, config.auth.token.clone());

    // JWT secret: prefer SYNOTE_JWT_SECRET env var, fall back to auth token
    let jwt_secret =
        std::env::var("SYNOTE_JWT_SECRET").unwrap_or_else(|_| config.auth.token.clone());

    let app_state = AppState::new(
        storage.clone(),
        links_index.clone(),
        tags_index.clone(),
        auth_config.clone(),
        users.clone(),
        jwt_secret,
    );

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Public routes — no auth required
    let public_routes = Router::new()
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/auth/status", get(auth_status))
        .route("/api/health", get(health_check))
        .route("/api/ready", get(readiness_check));

    // Protected routes — require Bearer token or valid JWT
    let protected_routes = Router::new()
        .route("/api/notes", get(list_notes))
        .route("/api/notes", post(create_note))
        .route("/api/notes/:id", get(get_note))
        .route("/api/notes/:id", put(update_note))
        .route("/api/notes/:id", delete(delete_note))
        .route("/api/search", get(search_notes))
        .route("/api/notes/:id/links", get(get_note_links))
        .route("/api/tags", get(list_tags))
        .route("/api/tags/:tag/notes", get(get_tagged_notes))
        .route("/api/notes/:id/tags", get(get_note_tags))
        .route("/api/graph", get(get_graph))
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            auth_middleware,
        ));

    let app = public_routes
        .merge(protected_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Synote server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
