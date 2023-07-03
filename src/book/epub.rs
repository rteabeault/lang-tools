use anyhow::anyhow;
use std::{path::PathBuf, io::{Read, Seek}};

use epub::doc::EpubDoc;

use super::Chapter;

impl Chapter {
    pub fn from_epub<R: Read + Seek>(
        mut epub: EpubDoc<R>,
        book_title: &str,
        base_path: &PathBuf,
    ) -> Vec<Chapter> {
        let chapters: Vec<(String, PathBuf)> = epub
            .toc
            .iter()
            .map(|c| (c.label.to_owned(), c.content.to_owned()))
            .collect();

        chapters
            .iter()
            .filter_map(|x| {
                // Filters out chapters where resource string is None 
                epub.get_resource_str_by_path(&x.1).map(|content| {
                    Chapter::new(
                        book_title.to_owned(),
                        x.0.to_owned(),
                        html2text::from_read(content.as_bytes(), std::usize::MAX),
                        base_path.to_owned())
                })
            })
            .collect()
    }
}

pub fn get_book_title(
    epub: &EpubDoc<std::io::BufReader<std::fs::File>>,
) -> Result<String, anyhow::Error> {
    return epub
        .mdata("title")
        .ok_or_else(|| anyhow!("Unable to derive book title from epub."));
}
