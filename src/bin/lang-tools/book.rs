use std::{path::PathBuf, ffi::OsStr};

use anyhow::Context;
use console::style;
use dialoguer::{Confirm, Select};
use lang_tools::{
    book::{write_toc, Chapter},
    clipboard::{get_clipboard, set_clipboard},
    file::write_to_file,
    translation::Translation,
};
use tabled::{settings::Style, Table};

use crate::common::{dialoguer_theme, print_info};

fn find_first_missing_chapter(chapters: &[Chapter]) -> Option<usize> {
    return chapters.iter().position(|c| !c.path.exists());
}

fn style_book_chapter(chapter: &Chapter) -> Result<String, anyhow::Error> {
    if chapter.path.exists() {
        Ok(style(format!(
            "{} Chapter: {} [{} - {} bytes]",
            "✔",
            chapter.chapter_title,
            chapter.path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string(),
            chapter.path.metadata()?.len()
        ))
        .cyan()
        .to_string())
    } else {
        Ok(style(format!("{} Chapter: {}", "✘", chapter.chapter_title))
            .red()
            .to_string())
    }
}

pub fn prompt_book_translation(
    toc_path: &PathBuf,
    book_title: &str,
    chapters: &[Chapter],
) -> Result<(), anyhow::Error> {

    print_info("Starting book translation session!");
    
    while let Some(selection) = prompt_for_chapters_list(&chapters)? {
        prompt_translate_book_chapter(&chapters[selection])?;
        write_toc(&toc_path, &book_title, &chapters)?;
    }

    Ok(())
}

fn prompt_translate_book_chapter(chapter: &Chapter) -> Result<(), anyhow::Error> {
    set_clipboard(&chapter.content)?;

    if Confirm::with_theme(&dialoguer_theme())
        .with_prompt(format!(
            "Chapter {} content has been placed in the paste buffer.\n  \
                Tranlate the content and then copy these translations into the paste buffer.\n  \
                Press 'y' to proceed or 'n' to cancel?",
            chapter.chapter_title
        ))
        .interact()?
    {
        let translation = get_clipboard()?;

        let translations = 
            Translation::from_source_and_target(&chapter.content, &translation)?;

        let table = 
            Table::new(translations).with(Style::markdown()).to_string();

        write_to_file(&chapter.path, &table)?;
    }

    Ok(())
}

fn prompt_for_chapters_list(chapters: &[Chapter]) -> Result<Option<usize>, anyhow::Error> {
    let items = chapters
        .iter()
        .map(style_book_chapter)
        .collect::<Result<Vec<String>, anyhow::Error>>()?;

    return Select::with_theme(&dialoguer_theme())
        .with_prompt("Pick a chapter to translate and press enter. Press 'esc' or 'q' to quit.")
        .items(&items)
        .default(find_first_missing_chapter(chapters).unwrap_or(0))
        .interact_opt()
        .context("Failed to create chapter prompt.");
}
