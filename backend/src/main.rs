mod api;
mod config;
mod links;
mod models;
// mod search; // Disabled - Tantivy removed temporarily
mod state;
mod storage;
// mod sync; // Disabled - git2 compilation errors
mod tags;

use crate::api::graph::get_graph;
use crate::api::health::{health_check, readiness_check};
use crate::api::links::get_note_links;
use crate::api::notes::{create_note, delete_note, get_note, list_notes, search_notes, update_note};
use crate::api::tags::{get_note_tags, get_tagged_notes, list_tags};
use crate::config::Config;
use crate::links::LinksIndex;
use crate::state::AppState;
use crate::storage::NoteStorage;
use crate::tags::TagIndex;
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

    let links_index = Arc::new(LinksIndex::new());
    let tags_index = Arc::new(TagIndex::new());

    let app_state = AppState::new(storage.clone(), links_index.clone(), tags_index.clone());

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/ready", get(readiness_check))
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
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Synote server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
