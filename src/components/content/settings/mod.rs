use std::path::Path;
use anyhow::Result;
use directories;
mod config;
mod method;
mod theme;
use std::fs::create_dir_all;
use anyhow::anyhow;
use crate::backend::storage::dots_storage_path;
use config::Config;

pub fn init() -> Result<Config> {
    let config_path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("config.json");
    let config = config::Config::load_or_create(&config_path);
    Ok(config)
}
