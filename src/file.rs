use std::{path::PathBuf, fs::{self, File}, io::Write};

use anyhow::Context;

use crate::path::expand_path;

pub fn read_from_file(file: PathBuf) -> Result<String, anyhow::Error> {
    let file = expand_path(&file)?;
    return fs::read_to_string(&file).context(format!("Failed reading from file {:?}.", file))
}

pub fn write_to_file(file: &PathBuf, text: &str) -> Result<(), anyhow::Error> {
    let file = expand_path(&file)?;

    if let Some(parent) = file.parent() {
        std::fs::create_dir_all(parent)
            .context(format!("Failed to create missing paths while writing file [{:?}]", file))?;
    }

    let mut file = File::create(file)?;
    
    return file.write_all(text.as_bytes()).context(format!("Failed writing to file {:?}.", file));
}