use srtlib::Subtitles;
use std::{env, path::PathBuf};

use anyhow::{anyhow, Context};

pub fn load_from_path(path: &PathBuf) -> Result<Subtitles, anyhow::Error> {
    return Subtitles::parse_from_file(path, None)
        .context(format!("Failed to read subtitles at [{:?}]", path));
}

pub fn build_subtitle_path_from_path(
    path: PathBuf,
    arg_lang: Option<String>,
    config_lang: Option<String>,
) -> PathBuf {
    let mut extension = arg_lang
        .or(config_lang)
        .unwrap_or("translated".to_owned());

    extension.push_str(".srt");

    let mut path = path.to_path_buf();

    path.set_extension("");
    path.set_extension(extension);
    return path
}

pub fn build_subtitle_path(
    arg_file: Option<PathBuf>,
    arg_path: Option<PathBuf>,
    config_path: Option<PathBuf>,
    title: &str,
    lang: &Option<String>,
    channel: &Option<String>,
) -> Result<PathBuf, anyhow::Error> {
    if let Some(file) = arg_file {
        if let Some(ext) = file.extension() {
            if ext == "srt" {
                return Ok(file);
            } else {
                return Err(anyhow!("File must end with an srt extension!"));
            }
        } else {
            return Err(anyhow!("File must end with an srt extension!"));
        }
    } else if let Some(path) = arg_path {
        return Ok(subtitle_sub_path(path, title, lang, channel));
    } else if let Some(path) = config_path {
        return Ok(subtitle_sub_path(path, title, lang, channel));
    } else {
        let path = env::current_dir().context("Unable to determine current directory")?;
        return Ok(subtitle_sub_path(path, title, lang, channel));
    }
}

fn subtitle_sub_path(
    base_path: PathBuf,
    title: &str,
    lang: &Option<String>,
    channel: &Option<String>,
) -> PathBuf {
    let mut subtitle_path = PathBuf::new();

    subtitle_path.push(base_path);

    if let Some(channel) = channel {
        subtitle_path.push(channel);
    }

    match lang {
        Some(lang) => subtitle_path.push(format!("{}.{}.srt", title, lang)),
        None => subtitle_path.push(format!("{}.srt", title)),
    }

    return subtitle_path;
}
