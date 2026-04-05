use crate::links::{LinksIndex, NoteLinks};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct LinksResponse {
    pub outgoing: Vec<LinkDto>,
    pub incoming: Vec<LinkDto>,
}

#[derive(Serialize)]
pub struct LinkDto {
    pub target_id: Option<String>,
    pub target_title: String,
    pub display_text: String,
    pub heading: Option<String>,
    pub exists: bool,
}

impl From<&crate::links::Link> for LinkDto {
    fn from(link: &crate::links::Link) -> Self {
        Self {
            target_id: link.target_id.map(|id| id.to_string()),
            target_title: link.target_title.clone(),
            display_text: link.display_text.clone(),
            heading: link.heading.clone(),
            exists: link.target_id.is_some(),
        }
    }
}

pub async fn get_note_links(
    Path(id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<Json<LinksResponse>, axum::http::StatusCode> {
    // Get all notes to build links and resolve targets
    let all_notes = app_state.storage.list().map_err(|_| axum::http::StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Update index for the current note
    if let Ok(note) = app_state.storage.get(&id) {
        app_state.links_index.update_note(&note, &all_notes);
    }
    
    // Get links for this note
    let note_links = app_state.links_index.get_note_links(id, &all_notes);
    
    let response = LinksResponse {
        outgoing: note_links.outgoing.iter().map(LinkDto::from).collect(),
        incoming: note_links.incoming.iter().map(LinkDto::from).collect(),
    };
    
    Ok(Json(response))
}
