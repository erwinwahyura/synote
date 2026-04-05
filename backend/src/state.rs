use crate::auth::AuthConfig;
use crate::links::LinksIndex;
use crate::search::SearchIndex;
use crate::storage::NoteStorage;
use crate::tags::TagIndex;
use std::sync::Arc;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<NoteStorage>,
    pub links_index: Arc<LinksIndex>,
    pub tags_index: Arc<TagIndex>,
    pub search_index: Option<Arc<SearchIndex>>,
    pub auth_config: AuthConfig,
}

impl AppState {
    pub fn new(
        storage: Arc<NoteStorage>,
        links_index: Arc<LinksIndex>,
        tags_index: Arc<TagIndex>,
        search_index: Option<Arc<SearchIndex>>,
        auth_config: AuthConfig,
    ) -> Self {
        Self {
            storage,
            links_index,
            tags_index,
            search_index,
            auth_config,
        }
    }
}
