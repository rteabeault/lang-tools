use std::path::PathBuf;

use anyhow::anyhow;
use dialoguer::Select;
use lang_tools::youtube::{self, SubtitleChoice, VideoInfo};
use url::Url;

use crate::common::{dialoguer_theme, print_bracketed_info};

pub static SUB_TRANSLATE_MSG: &'static str =
    "Copy subtitle translations to paste buffer and press 'y'. Or press 'n' to quit";

pub fn fetch_video_info(url: &Url) -> Result<VideoInfo, anyhow::Error> {
    print_bracketed_info("Fetching info for video", &url.as_str());

    let info = youtube::info(&url);

    if let Ok(info) = &info {
        print_bracketed_info("Found info for title", &info.name);
    }

    info
}

pub fn selecte_subtitle(
    lang: Option<String>,
    format: Option<String>,
    choices: &[SubtitleChoice],
) -> Result<SubtitleChoice, anyhow::Error> {
    match (lang, format) {
        (Some(lang), Some(format)) => choices
            .into_iter()
            .find(|choice| choice.lang == lang && choice.format == format)
            .ok_or(anyhow!("Failed to pick choice of subtitles"))
            .cloned(),
        _ => prompt_subtitle_choice(choices),
    }
}

pub fn prompt_subtitle_choice(choices: &[SubtitleChoice]) -> Result<SubtitleChoice, anyhow::Error> {
    Select::with_theme(&dialoguer_theme())
        .with_prompt("Choose a subtitle and press enter. Or hit 'esc' or 'q' to exit")
        .items(&choices)
        .default(0)
        .interact_opt()?
        .map(|selection| choices[selection].clone())
        .ok_or(anyhow!("No subtitle was selected"))
}

pub fn print_translated_subtitles_written_to(path: &PathBuf) {
    print_bracketed_info("Translated subtitles saved to", &path.to_string_lossy());
}

pub fn print_subtitles_written_to(path: &PathBuf) {
    print_bracketed_info("Subtitles saved to", &path.to_string_lossy());
}

