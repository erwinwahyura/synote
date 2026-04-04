use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub storage: StorageConfig,
    pub auth: AuthConfig,
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
pub struct AuthConfig {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default = "default_token")]
    pub token: String,
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

fn default_token() -> String {
    std::env::var("SYNOTE_AUTH_TOKEN").unwrap_or_else(|_| "changeme".to_string())
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
            auth: AuthConfig {
                enabled: false,
                token: default_token(),
            },
        }
    }
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        // Try to load from config.toml, fall back to defaults
        let config_path = PathBuf::from("config.toml");

        let mut config = if config_path.exists() {
            let content = std::fs::read_to_string(config_path)?;
            toml::from_str(&content)?
        } else {
            Config::default()
        };

        // Override token from environment if set
        if let Ok(token) = std::env::var("SYNOTE_AUTH_TOKEN") {
            config.auth.token = token;
            config.auth.enabled = true;
        }

        // Ensure auth is enabled if a non-default token is set
        if config.auth.token != "changeme" && !config.auth.token.is_empty() {
            config.auth.enabled = true;
        }

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
        assert!(!config.auth.enabled);
    }

    #[test]
    fn test_custom_token_enables_auth() {
        let toml_str = r#"
[auth]
token = "custom-secret-token"
"#;
        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.auth.enabled);
        assert_eq!(config.auth.token, "custom-secret-token");
    }
}
