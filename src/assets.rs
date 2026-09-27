use anyhow::Result;
use gpui_kit::{AssetSource, SharedString, assets::Assets};
use rust_embed::RustEmbed;
use std::borrow::Cow;

#[derive(RustEmbed)]
#[folder = "assets"]
pub struct AppAssets;

impl AssetSource for AppAssets {
  fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
    if let Some(file) = AppAssets::get(path) {
      return Ok(Some(file.data));
    }

    Assets.load(path)
  }

  fn list(&self, path: &str) -> Result<Vec<SharedString>> {
    let mut files = AppAssets::iter()
      .filter(|file| file.starts_with(path))
      .map(SharedString::from)
      .collect::<Vec<_>>();

    files.extend(Assets.list(path)?);

    Ok(files)
  }
}
