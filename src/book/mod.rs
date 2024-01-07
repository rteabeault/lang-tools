use std::path::PathBuf;

use fancy_regex::Regex;
use lazy_static::lazy_static;

pub mod epub;

lazy_static! {
    // Regex that matches a line that ends with an alphanumeric, comma, collon, or semi-colon.
    static ref EMPTY_LINE_RE: Regex = Regex::new(r"(?m)^\s*\n").unwrap();

    static ref HASH_LINE_RE: Regex = Regex::new(r"^\s*#\s*").unwrap();
}

#[derive(Debug)]
pub struct BookSection {
    pub title: String,
    pub index: usize,
    pub content: String,
}

impl BookSection {
    pub fn new(
        title: String,
        index: usize,
        content: String,
    ) -> Self {
        return BookSection { title, index, content };
    }
}

pub fn book_path(base_path: &PathBuf, book_title: &str) -> PathBuf {
    return base_path.join(book_title);
}

pub fn section_path(book_path: &PathBuf, section: &BookSection) -> PathBuf {
    book_path.join(format!("{:0>3}-{}.md", section.index, section.title))
}

pub fn clean_text(s: &str) -> String {
    let cleaned = EMPTY_LINE_RE.replace_all(&s, "");
    cleaned.to_string()
}




