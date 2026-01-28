use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub path: String, // Relative path within notes directory
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub path: Option<String>, // Optional folder path
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteRequest {
    pub title: Option<String>,
    pub content: Option<String>,
}

impl Note {
    pub fn new(title: String, content: String, path: Option<String>) -> Self {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let path = path.unwrap_or_else(|| format!("{}.md", id));

        Self {
            id,
            title,
            content,
            created_at: now,
            updated_at: now,
            path,
        }
    }

    pub fn update(&mut self, title: Option<String>, content: Option<String>) {
        if let Some(new_title) = title {
            self.title = new_title;
        }
        if let Some(new_content) = content {
            self.content = new_content;
        }
        self.updated_at = Utc::now();
    }
}
