mod api;
mod auth;
mod config;
mod models;
mod storage;
mod sync;

use crate::api::health::{health_check, readiness_check};
use crate::api::notes::{create_note, delete_note, get_note, list_notes, search_notes, update_note};
use crate::auth::{auth_middleware, AuthConfig};
use crate::config::Config;
use crate::storage::NoteStorage;
use crate::sync::GitSync;
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
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "synote=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::load()?;
    tracing::info!("Configuration loaded: {:?}", config);

    // Initialize git sync (if enabled)
    let git_sync = if config.sync.enabled {
        let notes_dir = config.storage.notes_dir.to_string_lossy().to_string();
        match GitSync::init(&notes_dir, config.sync.git_remote.clone()) {
            Ok(sync) => {
                tracing::info!("Git sync initialized at {}", notes_dir);
                Some(Arc::new(sync))
            }
            Err(e) => {
                tracing::warn!("Failed to initialize git sync: {}. Continuing without sync.", e);
                None
            }
        }
    } else {
        tracing::info!("Git sync disabled");
        None
    };
    
    // Initialize storage with sync
    let storage = Arc::new(NoteStorage::new(config.storage.notes_dir, git_sync)?);
    tracing::info!("Note storage initialized");

    // Build auth config
    let auth_config = AuthConfig::new(config.auth.enabled, config.auth.token.clone());

    // Build our application with routes
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/ready", get(readiness_check))
        .route("/api/notes", get(list_notes))
        .route("/api/notes", post(create_note))
        .route("/api/notes/:id", get(get_note))
        .route("/api/notes/:id", put(update_note))
        .route("/api/notes/:id", delete(delete_note))
        .route("/api/search", get(search_notes))
        .layer(middleware::from_fn_with_state(auth_config.clone(), auth_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(storage)
        .with_state(auth_config);

    // Run the server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Synote server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
