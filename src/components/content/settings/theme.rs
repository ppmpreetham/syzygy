use anyhow::{Result, anyhow};
use gpui_kit::{
    App, SharedString,
    component::{ActiveTheme, Theme, ThemeRegistry},
};
use log::error;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::from_utf8};
use strum::{Display, EnumIter, IntoEnumIterator};
use gpui_kit::gpui::WindowBackgroundAppearance;

use crate::backend::storage::dots_storage_path;

// radio
#[derive(Deserialize, Serialize, EnumIter, Display, Default, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
    #[default]
    System,
}

#[derive(Deserialize, Serialize, EnumIter, Display, Default, Clone, Copy, PartialEq, Eq)]
pub enum WindowStyle {
    #[default]
    Opaque,
    Transparent,
    Blurred,
    #[cfg(target_os = "windows")]
    MicaBackdrop,
    #[cfg(target_os = "windows")]
    MicaAltBackdrop,
}

impl Into<WindowBackgroundAppearance> for WindowStyle {
    fn into(self) -> WindowBackgroundAppearance {
        match self {
            Self::Opaque => WindowBackgroundAppearance::Opaque,
            Self::Transparent => WindowBackgroundAppearance::Transparent,
            Self::Blurred => WindowBackgroundAppearance::Blurred,
            #[cfg(target_os = "windows")]
            Self::MicaBackdrop => WindowBackgroundAppearance::MicaBackdrop,
            #[cfg(target_os = "windows")]
            Self::MicaAltBackdrop => WindowBackgroundAppearance::MicaAltBackdrop,
        }
    }
}


#[derive(RustEmbed)]
#[folder = "assets\\themes"]
#[include = "*.json"]
struct BundledThemes;

fn load_bundled_themes(cx: &mut App) {
    let registry = ThemeRegistry::global_mut(cx);

    for file in BundledThemes::iter() {
        let embedded = BundledThemes::get(&file);

        if let Some(asset) = &embedded
            && let Ok(content) = from_utf8(&asset.data)
            &&let Err(e) = registry.load_themes_from_str(content) {
                error!("Failed to load theme {file}: {e}");
        }
    }
}

fn apply_theme_by_name(name: &SharedString, cx: &mut App) {
    let Some(theme) = ThemeRegistry::global(cx).themes().get(name).cloned() else {
        return;
    };
    Theme::change(theme.mode, None, cx);
    Theme::update(cx, |current| current.apply_config(&theme));
}

pub fn theme_init(cx: &mut App) -> Result<()> {
    let theme_name = SharedString::from(cx.global::<crate::components::content::settings::config::Config>().theme_name.clone());

    load_bundled_themes(cx);
    apply_theme_by_name(&theme_name, cx);

    // user's custom overriding themes
    let path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find storage path"))?
        .join("themes/");

    if let Err(err) = ThemeRegistry::watch_dir(path, cx, move |cx| {
        load_bundled_themes(cx);
        apply_theme_by_name(&theme_name, cx);
    }) {
        error!("Failed to watch themes: {err}");
    }
    Ok(())
}


