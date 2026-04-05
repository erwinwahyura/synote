use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize)]
pub struct TagsListResponse {
    pub tags: Vec<TagInfo>,
}

#[derive(Serialize)]
pub struct TagInfo {
    pub name: String,
    pub count: usize,
}

#[derive(Serialize)]
pub struct TaggedNotesResponse {
    pub tag: String,
    pub notes: Vec<NoteSummary>,
}

#[derive(Serialize)]
pub struct NoteSummary {
    pub id: String,
    pub title: String,
}

#[derive(Deserialize)]
pub struct ListTagsQuery {
    #[serde(default)]
    pub limit: Option<usize>,
}

/// GET /api/tags - List all tags with counts
pub async fn list_tags(
    State(app_state): State<AppState>,
    Query(query): Query<ListTagsQuery>,
) -> Result<Json<TagsListResponse>, axum::http::StatusCode> {
    // Index all notes first (ensures fresh data)
    let all_notes = app_state.storage.list().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    for note in &all_notes {
        app_state.tags_index.index_note(note);
    }
    
    let mut tags = app_state.tags_index.get_all_tags();
    
    // Apply limit if specified
    if let Some(limit) = query.limit {
        tags.truncate(limit);
    }
    
    let response = TagsListResponse {
        tags: tags.into_iter()
            .map(|(name, count)| TagInfo { name, count })
            .collect(),
    };
    
    Ok(Json(response))
}

/// GET /api/tags/:tag/notes - Get all notes with a specific tag
pub async fn get_tagged_notes(
    Path(tag): Path<String>,
    State(app_state): State<AppState>,
) -> Result<Json<TaggedNotesResponse>, axum::http::StatusCode> {
    let all_notes = app_state.storage.list().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Index to ensure fresh data
    for note in &all_notes {
        app_state.tags_index.index_note(note);
    }
    
    let note_ids = app_state.tags_index.get_notes_with_tag(&tag.to_lowercase());
    
    let notes: Vec<NoteSummary> = note_ids
        .iter()
        .filter_map(|id| {
            app_state.storage.get(id).ok().map(|note| NoteSummary {
                id: note.id.to_string(),
                title: note.title,
            })
        })
        .collect();
    
    let response = TaggedNotesResponse {
        tag: tag.to_lowercase(),
        notes,
    };
    
    Ok(Json(response))
}

/// GET /api/notes/:id/tags - Get tags for a specific note
#[derive(Serialize)]
pub struct NoteTagsResponse {
    pub note_id: String,
    pub tags: Vec<String>,
}

pub async fn get_note_tags(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<Json<NoteTagsResponse>, axum::http::StatusCode> {
    // Get the note
    let note = app_state.storage.get(&id).map_err(|_| axum::http::StatusCode::NOT_FOUND)?;
    
    // Index it to extract tags
    app_state.tags_index.index_note(&note);
    
    let tags = app_state.tags_index.get_note_tags(&id);
    
    let response = NoteTagsResponse {
        note_id: id.to_string(),
        tags,
    };
    
    Ok(Json(response))
}
