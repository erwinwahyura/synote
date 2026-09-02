use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub sync: SyncConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    #[serde(default = "default_notes_dir")]
    pub notes_dir: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_git_remote")]
    pub git_remote: Option<String>,
}

fn default_git_remote() -> Option<String> {
    std::env::var("SYNOTE_GIT_REMOTE").ok()
}

fn default_host() -> String {
    "127.0.0.1".to_string()
}

fn default_port() -> u16 {
    8080
}

fn default_notes_dir() -> PathBuf {
    PathBuf::from("./data/notes")
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: default_host(),
                port: default_port(),
            },
            storage: StorageConfig {
                notes_dir: default_notes_dir(),
            },
            sync: SyncConfig {
                enabled: default_git_remote().is_some(),
                git_remote: default_git_remote(),
            },
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // Try to load from config.toml, fall back to defaults
        let config_path = PathBuf::from("config.toml");

        let config = if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            toml::from_str(&content)?
        } else {
            Config::default()
        };

        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.server.port, 8080);
        assert_eq!(config.server.host, "127.0.0.1");
    }
}
