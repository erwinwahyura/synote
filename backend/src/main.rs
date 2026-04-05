mod api;
mod auth;
mod config;
mod links;
mod models;
mod search;
mod state;
mod storage;
mod sync;
mod tags;

use crate::api::graph::get_graph;
use crate::api::health::{health_check, readiness_check};
use crate::api::links::get_note_links;
use crate::api::notes::{create_note, delete_note, get_note, list_notes, search_notes, update_note};
use crate::api::tags::{get_note_tags, get_tagged_notes, list_tags};
use crate::auth::{auth_middleware, AuthConfig};
use crate::config::Config;
use crate::links::LinksIndex;
use crate::search::SearchIndex;
use crate::state::AppState;
use crate::storage::NoteStorage;
use crate::sync::GitSync;
use crate::tags::TagIndex;
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

    // Initialize storage (git sync disabled for now - needs thread-safe implementation)
    let notes_dir = config.storage.notes_dir.clone();
    let storage = Arc::new(NoteStorage::new(notes_dir)?);
    tracing::info!("Note storage initialized");
    
    // Initialize links index for wikilinks/backlinks
    let links_index = Arc::new(LinksIndex::new());
    tracing::info!("Links index initialized");
    
    // Initialize tags index for #tag tracking
    let tags_index = Arc::new(TagIndex::new());
    tracing::info!("Tags index initialized");
    
    // Initialize Tantivy search index
    let search_index: Option<Arc<SearchIndex>> = {
        let index_dir = config.storage.notes_dir.join(".tantivy");
        match SearchIndex::open(&index_dir) {
            Ok(index) => {
                tracing::info!("Search index initialized at {:?}", index_dir);
                let index = Arc::new(index);
                
                // Index all existing notes
                if let Ok(all_notes) = storage.list() {
                    tracing::info!("Indexing {} notes for search...", all_notes.len());
                    for note in &all_notes {
                        if let Err(e) = index.index_note(note) {
                            tracing::warn!("Failed to index note {}: {}", note.id, e);
                        }
                    }
                    if let Err(e) = index.commit() {
                        tracing::warn!("Failed to commit search index: {}", e);
                    } else {
                        tracing::info!("Search index populated successfully");
                    }
                }
                
                Some(index)
            }
            Err(e) => {
                tracing::warn!("Failed to initialize search index: {}. Continuing without full-text search.", e);
                None
            }
        }
    };

    // Build auth config
    let auth_config = AuthConfig::new(config.auth.enabled, config.auth.token.clone());

    // Build unified app state
    let app_state = AppState::new(
        storage.clone(),
        links_index.clone(),
        tags_index.clone(),
        search_index.clone(),
        auth_config.clone(),
    );

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
        .route("/api/notes/:id/links", get(get_note_links))
        .route("/api/tags", get(list_tags))
        .route("/api/tags/:tag/notes", get(get_tagged_notes))
        .route("/api/notes/:id/tags", get(get_note_tags))
        .route("/api/graph", get(get_graph))
        .layer(middleware::from_fn_with_state(app_state.clone(), auth_middleware))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    // Run the server
    let addr = format!("{}:{}", config.server.host, config.server.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("Synote server listening on {}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
