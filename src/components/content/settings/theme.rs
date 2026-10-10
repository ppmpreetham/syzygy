use gpui_kit::{App, SharedString, component::{Theme, ThemeRegistry}};
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, IntoEnumIterator};
use std::path::PathBuf;
use log::error;
use anyhow::{Result, anyhow};

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


pub fn theme_init(cx: &mut App) -> Result<()>{
    let theme_name = SharedString::from("Ayu Dark");
    let path = dots_storage_path()
      .ok_or_else(|| anyhow!("Can't find the storage path"))?
      .join("themes/");

    if let Err(err) = ThemeRegistry::watch_dir(path, cx, move |cx| {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(&theme_name)
            .cloned()
        {
            Theme::update(cx, |current| current.apply_config(&theme));
        }
    }) {
        error!("Failed to watch themes directory: {err}");
    }
    Ok(())
}
