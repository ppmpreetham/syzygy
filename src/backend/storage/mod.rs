mod datastructure;
pub mod disk;
mod serializer;
mod utils;

use super::intercept::exchange::Exchange;

use std::path::PathBuf;
use directories::ProjectDirs;
pub fn storage_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "syzygy", "SyZyGy")
        .map(|proj_dirs| proj_dirs.config_dir().to_path_buf())
}
