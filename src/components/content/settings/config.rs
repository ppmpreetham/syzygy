use super::theme::{Theme, ThemeMode};
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};
use std::fs::File;
use anyhow::{Result, Error, Context};
use serde_json;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub theme: Theme,
    pub theme_mode: ThemeMode,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            theme: Theme::Monokai,
            theme_mode: ThemeMode::System,
        }
    }
}

impl Config {
  /// load config
    pub fn load(file_path: impl AsRef<Path>) -> Result<Self> {
        let content = fs::read_to_string(file_path)
            .context("Failed to read config file from disk")?;
        let config: Self = serde_json::from_str(&content)
            .context("Failed to parse the config file")?;
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
      Self::load(&file_path)
        .unwrap_or_else(|_| Self::create(file_path).unwrap_or_default())
    }
}
