use crate::models::{CreateNoteRequest, Note, UpdateNoteRequest};
use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn search_notes(
    State(app_state): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<Note>>, AppError> {
    // Use Tantivy if available, fall back to naive search
    if let Some(ref search_index) = app_state.search_index {
        let search_results = search_index.search(&params.q, 50)
            .map_err(AppError::from)?;
        
        // Fetch full notes for search results
        let mut notes = Vec::new();
        for result in search_results {
            if let Ok(id) = uuid::Uuid::parse_str(&result.id) {
                if let Ok(note) = app_state.storage.get(&id) {
                    notes.push(note);
                }
            }
        }
        Ok(Json(notes))
    } else {
        // Fall back to naive storage search
        let results = app_state.storage.search(&params.q)?;
        Ok(Json(results))
    }
}

pub async fn list_notes(
    State(app_state): State<AppState>,
) -> Result<Json<Vec<Note>>, AppError> {
    let notes = app_state.storage.list()?;
    Ok(Json(notes))
}

pub async fn get_note(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>, AppError> {
    let note = app_state.storage.get(&id)?;
    Ok(Json(note))
}

pub async fn create_note(
    State(app_state): State<AppState>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    let note = Note::new(req.title, req.content, req.path);
    let note = app_state.storage.create(note)?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn update_note(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<Json<Note>, AppError> {
    let mut note = app_state.storage.get(&id)?;
    note.update(req.title, req.content);
    let note = app_state.storage.update(&id, note)?;
    Ok(Json(note))
}

pub async fn delete_note(
    State(app_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    app_state.storage.delete(&id)?;
    Ok(StatusCode::NO_CONTENT)
}

// Error handling
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Error: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
