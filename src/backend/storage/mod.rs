mod datastructure;
pub mod disk;
mod serializer;
mod utils;

use super::intercept::exchange::Exchange;

use std::path::PathBuf;
use directories::ProjectDirs;
fn project_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "syzygy", "SyZyGy")
}

pub fn dots_storage_path() -> Option<PathBuf> {
    project_dirs().map(|dirs| dirs.config_dir().to_path_buf())
}

pub fn local_data_storage_path() -> Option<PathBuf> {
    project_dirs().map(|dirs| dirs.data_local_dir().to_path_buf())
}
