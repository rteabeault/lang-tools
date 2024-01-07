use anyhow::anyhow;
use fancy_regex::Regex;
use std::io::{Read, Seek};
use epub::doc::EpubDoc;
use lazy_static::lazy_static;
use crate::book::clean_text;

use super::BookSection;

lazy_static! {
    // Regex that matches a line that has a # at the beginning.
    static ref HASH_LINE_RE: Regex = Regex::new(r"^\s*#\s*").unwrap();
}

impl BookSection {
    pub fn from_epub<R: Read + Seek>(
        mut epub: EpubDoc<R>,
    ) -> Vec<BookSection> {
        let mut sections: Vec<BookSection> = vec![] ;
        loop {
            if let Some((content, _)) = epub.get_current() {
                let content = html2text::from_read(content.as_slice(), std::usize::MAX);
                let content = clean_text(&content);

                if content.len() > 0 {
                    let section_title = content.lines().next().unwrap();
                    let section_title = clean_section_title(&section_title);

                    let section = BookSection::new(
                        section_title.to_owned(),
                        epub.get_current_page(),
                        content.trim().to_owned());

                    sections.push(section);
                }
            }

            if !epub.go_next() {
                break;
            }
        }

        sections
    }
}

pub fn get_book_title(
    epub: &EpubDoc<std::io::BufReader<std::fs::File>>,
) -> Result<String, anyhow::Error> {
    return epub
        .mdata("title")
        .ok_or_else(|| anyhow!("Unable to derive book title from epub."));
}

fn clean_section_title(s: &str) -> String {
    HASH_LINE_RE.replace_all(s, "").to_string()
}