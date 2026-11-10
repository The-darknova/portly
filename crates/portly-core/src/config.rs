use crate::error::CoreError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortlyConfig {
    #[serde(default)]
    pub preferred_editor: Option<String>,
    #[serde(default)]
    pub preferred_terminal: Option<String>,
    #[serde(default = "default_auto_refresh")]
    pub auto_refresh_secs: u64,
    #[serde(default)]
    pub use_https: bool,
    #[serde(default)]
    pub pinned_ports: Vec<u16>,
}

fn default_auto_refresh() -> u64 {
    2
}

impl Default for PortlyConfig {
    fn default() -> Self {
        Self {
            preferred_editor: None,
            preferred_terminal: None,
            auto_refresh_secs: 2,
            use_https: false,
            pinned_ports: Vec::new(),
        }
    }
}

pub fn get_config_path() -> PathBuf {
    if let Some(mut dir) = dirs::config_dir() {
        dir.push("portly");
        dir.push("config.toml");
        dir
    } else {
        PathBuf::from(".portly.toml")
    }
}

pub fn load_config() -> PortlyConfig {
    let path = get_config_path();
    if !path.exists() {
        return PortlyConfig::default();
    }

    match fs::read_to_string(&path) {
        Ok(content) => match toml::from_str(&content) {
            Ok(cfg) => cfg,
            Err(e) => {
                tracing::warn!("Failed to parse config file at {:?}: {}", path, e);
                PortlyConfig::default()
            }
        },
        Err(e) => {
            tracing::warn!("Failed to read config file at {:?}: {}", path, e);
            PortlyConfig::default()
        }
    }
}

pub fn save_config(config: &PortlyConfig) -> Result<(), CoreError> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| CoreError::ActionFailed(format!("Failed to create config dir: {e}")))?;
    }

    let toml_str = toml::to_string_pretty(config)
        .map_err(|e| CoreError::ActionFailed(format!("Failed to serialize config: {e}")))?;

    fs::write(&path, toml_str)
        .map_err(|e| CoreError::ActionFailed(format!("Failed to write config file: {e}")))?;

    Ok(())
}
