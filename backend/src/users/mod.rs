use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
}

pub struct UserStorage {
    users_file: PathBuf,
    users: RwLock<Vec<User>>,
}

impl UserStorage {
    pub fn new(data_dir: &Path) -> Result<Self> {
        std::fs::create_dir_all(data_dir)?;
        let users_file = data_dir.join("users.json");
        let users: Vec<User> = if users_file.exists() {
            let content = std::fs::read_to_string(&users_file)?;
            serde_json::from_str(&content).unwrap_or_default()
        } else {
            vec![]
        };
        Ok(Self {
            users_file,
            users: RwLock::new(users),
        })
    }

    pub fn count(&self) -> usize {
        self.users.read().unwrap().len()
    }

    pub fn find_by_username(&self, username: &str) -> Option<User> {
        self.users
            .read()
            .unwrap()
            .iter()
            .find(|u| u.username.to_lowercase() == username.to_lowercase())
            .cloned()
    }

    pub fn create(&self, username: String, password_hash: String) -> Result<User> {
        let user = User {
            id: Uuid::new_v4(),
            username,
            password_hash,
            created_at: Utc::now(),
        };
        let mut users = self.users.write().unwrap();
        users.push(user.clone());
        let content = serde_json::to_string_pretty(&*users)?;
        std::fs::write(&self.users_file, content)?;
        Ok(user)
    }
}
