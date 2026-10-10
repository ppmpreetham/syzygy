mod config;
pub mod menu;
pub mod theme;

use anyhow::Result;
use anyhow::anyhow;
use config::Config;
use directories;
use gpui_kit::App;
use gpui_kit::component::Theme;
use std::fs::create_dir_all;
use std::fs::write;
use std::path::Path;

use crate::backend::storage::dots_storage_path;
use crate::components::content::settings::theme::ThemeMode;

pub fn init() -> Result<Config> {
    let config_path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("config.json");
    let config = config::Config::load_or_create(&config_path);
    Ok(config)
}

use gpui_kit::component::ThemeMode as GpuiThemeMode;

pub fn config_init(cx: &mut App) -> Result<()> {
    let config_path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("config.json");

    let config = Config::load_or_create(&config_path);
    match config.theme_mode {
        ThemeMode::System => Theme::sync_system_appearance(None, cx),
        ThemeMode::Light => Theme::change(GpuiThemeMode::Light, None, cx),
        ThemeMode::Dark => Theme::change(GpuiThemeMode::Dark, None, cx),
    }

    let path = config_path.to_path_buf();
    cx.set_global(config);
    cx.observe_global::<Config>(move |cx| {
        if let Ok(json) = serde_json::to_string_pretty(cx.global::<Config>()) {
            write(&path, json).ok();
        }
    }).detach();
    Ok(())
}
