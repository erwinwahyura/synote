use crate::links::LinksIndex;
use crate::storage::NoteStorage;
use crate::tags::TagIndex;
use std::sync::Arc;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<NoteStorage>,
    pub links_index: Arc<LinksIndex>,
    pub tags_index: Arc<TagIndex>,
}

impl AppState {
    pub fn new(
        storage: Arc<NoteStorage>,
        links_index: Arc<LinksIndex>,
        tags_index: Arc<TagIndex>,
    ) -> Self {
        Self {
            storage,
            links_index,
            tags_index,
        }
    }
}
