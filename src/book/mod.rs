use std::path::PathBuf;

use itertools::Itertools;

use crate::file::write_to_file;

pub mod epub;

#[derive(Debug)]
pub struct Chapter {
    pub chapter_title: String,
    pub content: String,
    pub path: PathBuf,
}

impl Chapter {
    pub fn new(
        book_title: String,
        chapter_title: String,
        content: String,
        base_path: PathBuf,
    ) -> Self {
        let path = base_path.join(format!("{}-{}.md", book_title, chapter_title));
        return Chapter {
            chapter_title,
            content,
            path,
        };
    }
}

pub fn book_path(base_path: &PathBuf, book_title: &str) -> PathBuf {
    return base_path.join(book_title);
}

pub fn toc_path(book_path: &PathBuf, book_title: &str) -> PathBuf {
    book_path.join(format!("{}.md", book_title))
}

// If there are chapters with a file path that exists then write the TOC.
pub fn write_toc(path: &PathBuf, book_title: &str, chapters: &[Chapter]) -> Result<(), anyhow::Error> {
    let toc_chapters = chapters
        .iter()
        .filter(|c| c.path.exists())
        .map(|c| {
            format!(
                "[{}]({})",
                c.chapter_title,
                c.path.file_name().unwrap().to_string_lossy()
            )
        })
        .join("\n");

    if toc_chapters.len() > 0 {
        let toc = format!("# {}\n{}", book_title, toc_chapters);

        write_to_file(path, &toc)
    } else {
        Ok(())
    }
}


