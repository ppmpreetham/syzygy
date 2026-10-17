pub mod config;
pub mod menu;
pub mod theme;

use anyhow::Result;
use anyhow::anyhow;
use config::Config;
use gpui_kit::App;
use std::fs::write;

use crate::backend::storage::dots_storage_path;

pub fn init() -> Result<Config> {
    let config_path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("config.json");
    let config = config::Config::load_or_create(&config_path);
    Ok(config)
}

pub fn config_init(cx: &mut App) -> Result<()> {
    let config_path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("config.json");

    let config = Config::load_or_create(&config_path);
    let path = config_path.to_path_buf();
    cx.set_global(config);
    cx.observe_global::<Config>(move |cx| {
        if let Ok(json) = serde_json::to_string_pretty(cx.global::<Config>()) {
            write(&path, json).ok();
        }
    }).detach();
    Ok(())
}
