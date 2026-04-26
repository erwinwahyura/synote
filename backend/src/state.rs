use crate::auth::AuthConfig;
use crate::links::LinksIndex;
use crate::storage::NoteStorage;
use crate::tags::TagIndex;
use crate::users::UserStorage;
use std::sync::Arc;

/// Application state shared across all handlers
#[derive(Clone)]
pub struct AppState {
    pub storage: Arc<NoteStorage>,
    pub links_index: Arc<LinksIndex>,
    pub tags_index: Arc<TagIndex>,
    pub auth_config: AuthConfig,
    pub users: Arc<UserStorage>,
    pub jwt_secret: Arc<String>,
}

impl AppState {
    pub fn new(
        storage: Arc<NoteStorage>,
        links_index: Arc<LinksIndex>,
        tags_index: Arc<TagIndex>,
        auth_config: AuthConfig,
        users: Arc<UserStorage>,
        jwt_secret: String,
    ) -> Self {
        Self {
            storage,
            links_index,
            tags_index,
            auth_config,
            users,
            jwt_secret: Arc::new(jwt_secret),
        }
    }
}
