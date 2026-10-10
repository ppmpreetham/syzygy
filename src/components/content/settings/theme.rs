use gpui_kit::{App, SharedString, component::{ActiveTheme, Theme, ThemeRegistry}};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use std::{path::PathBuf, str::from_utf8};
use log::error;
use anyhow::{Result, anyhow};
use rust_embed::RustEmbed;

use crate::backend::storage::dots_storage_path;
use super::method::method;

// radio
#[derive(Deserialize, Serialize, EnumIter, Display)]
pub enum ThemeMode {
    Light,
    Dark,
    System,
}

impl ThemeMode {
    pub fn method() -> method {
        method::Radio(Self::iter().map(|v| v.to_string()).collect())
    }
}

// list
pub fn theme_options(cx: &App) -> Vec<SharedString> {
    ThemeRegistry::global(cx)
        .sorted_themes()
        .into_iter()
        .map(|t| t.name.clone())
        .collect()
}



#[derive(RustEmbed)]
#[folder = "assets\\themes"]
#[include = "*.json"]
struct BundledThemes;

pub fn theme_init(cx: &mut App) -> Result<()> {
    let theme_name = SharedString::from("Ayu Dark");
    let registry = ThemeRegistry::global_mut(cx);
    for file in BundledThemes::iter() {
        if let Some(embedded) = BundledThemes::get(&file)
            && let Ok(content) = from_utf8(&embedded.data) && let Err(e) = registry.load_themes_from_str(content) {
                    error!("Failed to load bundled theme {file}: {e}");
                }
    }

    if let Some(theme) = ThemeRegistry::global(cx).themes().get(&theme_name).cloned() {
        Theme::change(theme.mode, None, cx);
        Theme::update(cx, |current| current.apply_config(&theme));
    }

    // user's custom overriding themes
    let path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find the storage path"))?
        .join("themes/");

    if let Err(err) = ThemeRegistry::watch_dir(path, cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned()
        {
            println!("Setting mode to: {:?}", theme.mode);
            Theme::change(theme.mode, None, cx);
            Theme::update(cx, |current| current.apply_config(&theme));
        }
    }) {
        error!("Failed to watch themes directory: {err}");
    }

    Ok(())
}


