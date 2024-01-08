use std::{path::PathBuf, ffi::OsStr};

use anyhow::Context;
use console::style;
use dialoguer::{Select, Editor};
use itertools::Itertools;
use lang_tools::{
    book::{BookSection, section_path, alternating_text_section_path},
    clipboard::set_clipboard,
    file::write_to_file,
    translation::Translation,
};
// use tabled::{settings::Style, Table};

use crate::common::{dialoguer_theme, print_info};

fn find_first_missing_section(base_path: &PathBuf, sections: &[BookSection]) -> Option<usize> {
    return sections.iter().position(|c| {
        let path = section_path(base_path, c);
        !path.exists()
    });
}

fn style_book_section(base_path: &PathBuf, section: &BookSection) -> Result<String, anyhow::Error> {
    let section_path = section_path(base_path, &section);
    if section_path.exists() {
        Ok(style(format!(
            "{} Section: {} [{} - {} bytes]",
            "✔",
            section.title,
            section_path.file_name().unwrap_or(OsStr::new("")).to_string_lossy().to_string(),
            section_path.metadata()?.len()
        ))
        .cyan()
        .to_string())
    } else {
        Ok(style(format!("{} Section: {}", "✘", section.title))
            .red()
            .to_string())
    }
}

pub fn prompt_book_translation(
    book_path: &PathBuf,
    sections: &mut [BookSection],
) -> Result<(), anyhow::Error> {

    print_info("Starting book translation session!");
    
    while let Some(selection) = prompt_for_sections_list(&book_path, &sections)? {
        prompt_translate_book_section(book_path, &mut sections[selection])?;
    }

    Ok(())
}   

fn prompt_translate_book_section(base_path: &PathBuf, section: &mut BookSection) -> Result<(), anyhow::Error> {
    set_clipboard(&section.content)?;

    if let Some(translation) = Editor::new().edit("").unwrap() {

        let translations = 
            Translation::from_source_and_target(&section.content, &translation)?;


        let alternating_langs = translations.iter().map(|t| {
            let mut text: String = "".to_string();
            text.push_str(format!("{}\n\n", &t.source_text.trim()).as_str());
            text.push_str(format!("**{}**", t.target_text.trim()).as_str());
            text
        }).join("\n\n---\n\n");

        let content = format!("# {}\n{}", section.title, alternating_langs);

        let path = alternating_text_section_path(&base_path, section);

        write_to_file(&path, &content)?;

        // let table = 
        //     Table::new(translations).with(Style::markdown()).to_string();

        // let content = format!("# {}\n {}", section.title, table);

        // let path = section_path(&base_path, section);

        // write_to_file(&path, &content)?;
    } else {
        println!("Translation was not saved.");
    }

    Ok(())
}

// fn write_markfown_table(translations: Vec<Translation>) -> Result<(), anyhow::Error> {
//     let table = 
//     Table::new(translations).with(Style::markdown()).to_string();

//     let content = format!("# {}\n {}", section.title, table);

//     let path = section_path(&base_path, section);

//     write_to_file(&path, &content)?;
// }

// fn write_markfown_alternating_paragraphs(translations: Vec<Translation>) -> Result<(), anyhow::Error> {
    
// }


fn prompt_for_sections_list(base_path: &PathBuf, sections: &[BookSection]) -> Result<Option<usize>, anyhow::Error> {
    let items = sections
        .iter()
        .map(|section| style_book_section(&base_path, section))
        .collect::<Result<Vec<String>, anyhow::Error>>()?;

    return Select::with_theme(&dialoguer_theme())
        .with_prompt("Pick a section to translate and press enter. The text will be placed in your paste buffer and an editor will be opened for you to place your translation. Press 'esc' or 'q' to quit.")
        .items(&items)
        .default(find_first_missing_section(&base_path, sections).unwrap_or(0))
        .interact_opt()
        .context("Failed to create section prompt.");
}
