use std::{path::PathBuf, str::FromStr, env};

use anyhow::Context;

pub fn expand_path(path: &PathBuf) -> Result<PathBuf, anyhow::Error> {
    let orig_path = path.to_string_lossy(); 
    let path = shellexpand::tilde(&orig_path);
    
    return PathBuf::from_str(&path.into_owned())
        .context(format!("Failed to expand path [{:?}]", orig_path))
}

/// Returns the first Some(path) in the vector or the current directory if it can be determined.
pub fn first_path_or_current_dir(paths: Vec<Option<PathBuf>>) -> Result<PathBuf, anyhow::Error> {
    match paths.into_iter().flatten().next() {
        Some(path) => return expand_path(&path),
        None => return env::current_dir().context("Unable to determine current directory"),
    }
}