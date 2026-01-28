use crate::models::{CreateNoteRequest, Note, UpdateNoteRequest};
use crate::storage::NoteStorage;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;
use uuid::Uuid;

pub type AppState = Arc<NoteStorage>;

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    q: String,
}

pub async fn search_notes(
    State(storage): State<AppState>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<Note>>, AppError> {
    let results = storage.search(&params.q)?;
    Ok(Json(results))
}

pub async fn list_notes(
    State(storage): State<AppState>,
) -> Result<Json<Vec<Note>>, AppError> {
    let notes = storage.list()?;
    Ok(Json(notes))
}

pub async fn get_note(
    State(storage): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Note>, AppError> {
    let note = storage.get(&id)?;
    Ok(Json(note))
}

pub async fn create_note(
    State(storage): State<AppState>,
    Json(req): Json<CreateNoteRequest>,
) -> Result<(StatusCode, Json<Note>), AppError> {
    let note = Note::new(req.title, req.content, req.path);
    let note = storage.create(note)?;
    Ok((StatusCode::CREATED, Json(note)))
}

pub async fn update_note(
    State(storage): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateNoteRequest>,
) -> Result<Json<Note>, AppError> {
    let mut note = storage.get(&id)?;
    note.update(req.title, req.content);
    let note = storage.update(&id, note)?;
    Ok(Json(note))
}

pub async fn delete_note(
    State(storage): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    storage.delete(&id)?;
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
