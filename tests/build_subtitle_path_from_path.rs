use std::{path::PathBuf, str::FromStr};

use lang_tools::subtitle::path::build_subtitle_path_from_path;

#[test]
fn should_return_path_with_lang_from_arg() {
    let path = PathBuf::from_str("/path/file.de.srt").unwrap();
    let result = build_subtitle_path_from_path(path, Some("en".to_owned()), None);
    assert_eq!(result, PathBuf::from_str("/path/file.en.srt").unwrap())
}

#[test]
fn should_return_path_with_lang_from_config() {
    let path = PathBuf::from_str("/path/file.de.srt").unwrap();
    let result = build_subtitle_path_from_path(path, None, Some("en".to_owned()));
    assert_eq!(result, PathBuf::from_str("/path/file.en.srt").unwrap())
}

#[test]
fn should_use_translate_when_missing_lang() {
    let path = PathBuf::from_str("/path/file.srt").unwrap();
    let result = build_subtitle_path_from_path(path, None, None);
    assert_eq!(result, PathBuf::from_str("/path/file.translated.srt").unwrap())
}