use anyhow::{Context, Result};
use arboard::Clipboard;

pub fn set_clipboard(text: &String) -> Result<(), anyhow::Error> {
    let mut clipboard = Clipboard::new()?;

    clipboard
        .set_text(text)
        .context("Failed to set clipboard contents.")
}

pub fn get_clipboard() -> Result<String, anyhow::Error> {
    let mut clipboard = Clipboard::new()?;
    return clipboard
        .get_text()
        .context("Failed to read clipboard contents.");
}