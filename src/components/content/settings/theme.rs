use anyhow::{Result, anyhow};
use gpui_kit::{
    App,
    component::{Theme, ThemeRegistry, ThemeMode as GpuiThemeMode},
};
use log::error;
use rust_embed::RustEmbed;
use serde::{Deserialize, Serialize};
use std::str::from_utf8;
use strum::{Display, EnumIter};
use gpui_kit::gpui::WindowBackgroundAppearance;
use crate::components::content::settings::config;
use crate::backend::storage::dots_storage_path;

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

impl From<WindowStyle> for WindowBackgroundAppearance {
    fn from(val: WindowStyle) -> Self {
        match val {
            WindowStyle::Opaque => WindowBackgroundAppearance::Opaque,
            WindowStyle::Transparent => WindowBackgroundAppearance::Transparent,
            WindowStyle::Blurred => WindowBackgroundAppearance::Blurred,
            #[cfg(target_os = "windows")]
            WindowStyle::MicaBackdrop => WindowBackgroundAppearance::MicaBackdrop,
            #[cfg(target_os = "windows")]
            WindowStyle::MicaAltBackdrop => WindowBackgroundAppearance::MicaAltBackdrop,
        }
    }
}

pub fn effective_mode(cx: &App) -> GpuiThemeMode {
    match cx.global::<config::Config>().theme_mode {
        ThemeMode::Light => GpuiThemeMode::Light,
        ThemeMode::Dark => GpuiThemeMode::Dark,
        ThemeMode::System => cx.window_appearance().into(),
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
            && let Err(e) = registry.load_themes_from_str(content)
        {
            error!("Failed to load theme {file}: {e}");
        }
    }
}

pub fn theme_init(cx: &mut App) -> Result<()> {
    load_bundled_themes(cx);

    let cfg = cx.global::<config::Config>();
    let light_name = cfg.light_theme_name.clone();
    let dark_name = cfg.dark_theme_name.clone();
    let reg = ThemeRegistry::global(cx);
    let light_theme = reg.themes().get(light_name.as_str()).cloned();
    let dark_theme = reg.themes().get(dark_name.as_str()).cloned();
    Theme::update(cx, |t| {
        if let Some(c) = light_theme {
            t.light_theme = c;
        }
        if let Some(c) = dark_theme {
            t.dark_theme = c;
        }
    });
    Theme::change(effective_mode(cx), None, cx);

    // user's custom overriding themes
    let path = dots_storage_path()
        .ok_or_else(|| anyhow!("Can't find storage path"))?
        .join("themes/");

    if let Err(err) = ThemeRegistry::watch_dir(path, cx, move |cx| {
        load_bundled_themes(cx);
    }) {
        error!("Failed to watch themes: {err}");
    }
    Ok(())
}
