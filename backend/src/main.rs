mod api;
mod config;
mod models;
mod storage;

use crate::api::notes::{create_note, delete_note, get_note, list_notes, search_notes, update_note};
use crate::config::Config;
use crate::storage::NoteStorage;
use axum::{
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

    // Initialize storage
    let storage = Arc::new(NoteStorage::new(config.storage.notes_dir)?);
    tracing::info!("Note storage initialized");

    // Build our application with routes
    let app = Router::new()
        .route("/api/notes", get(list_notes))
        .route("/api/notes", post(create_note))
        .route("/api/notes/:id", get(get_note))
        .route("/api/notes/:id", put(update_note))
        .route("/api/notes/:id", delete(delete_note))
        .route("/api/search", get(search_notes))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(storage);

    // Run the server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Synote server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
