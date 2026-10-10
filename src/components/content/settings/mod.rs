use anyhow::Result;
use directories;
use std::path::Path;
mod config;
mod method;
pub mod theme;
use crate::backend::storage::dots_storage_path;
use anyhow::anyhow;
use config::Config;
use std::fs::create_dir_all;

pub fn init() -> Result<Config> {
    let config_path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("config.json");
    let config = config::Config::load_or_create(&config_path);
    Ok(config)
}
