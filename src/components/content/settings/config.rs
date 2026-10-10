use anyhow::{Context, Error, Result};
use gpui_kit::App;
use gpui_kit::component::Theme;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::{fs, path::Path};
use gpui_kit::Global;
use super::theme::{ThemeMode, WindowStyle};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub theme_mode: ThemeMode,
    pub theme_name: String,
    pub window_style: WindowStyle,
}
impl Global for Config {}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme_mode: ThemeMode::System,
            theme_name: "Ayu Dark".to_string(),
            window_style: WindowStyle::Opaque,
        }
    }
}


impl Config {
    /// load config
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self> {
        let content =
            fs::read_to_string(file_path).context("Failed to read config file from disk")?;
        let config: Self =
            serde_json::from_str(&content).context("Failed to parse the config file")?;
        Ok(config)
    }

    /// creates actual file with default values
    pub fn create(path: &Path) -> Result<Self> {
        if let Some(p) = path.parent() {
            fs::create_dir_all(p).ok();
        }
        let config = Self::default();
        serde_json::to_writer_pretty(File::create(path)?, &config)?;
        Ok(config)
    }

    /// inspired from oncelock crate
    pub fn load_or_create(file_path: &Path) -> Self {
        Self::load(file_path).unwrap_or_else(|_| Self::create(file_path).unwrap_or_default())
    }
}



