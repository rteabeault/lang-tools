use std::path::PathBuf;

use anyhow::Context;
use srtlib::Subtitles;

use crate::path::expand_path;

pub fn write_subtitles(path: &PathBuf, subtitles: &Subtitles) -> Result<PathBuf, anyhow::Error> {
    let path = expand_path(path).context(format!("Unable to expand path [{:?}]", path))?;

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)
            .context(format!("Failed to create missing paths in path [{:?}]", parent))?;
    }
    

    subtitles.write_to_file(&path, None).map(|_| path)
        .context("Failed to save subtitles")
}