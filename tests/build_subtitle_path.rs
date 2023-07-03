use std::{str::FromStr, path::{Path, PathBuf}};
use lang_tools::subtitle::path::build_subtitle_path;

#[test]
fn should_return_file_path_if_it_has_srt_extension() {
    let path = build_subtitle_path(
        Some(PathBuf::from_str("/path/subs.srt").unwrap()),
        Some(PathBuf::from_str("/arg_path").unwrap()),
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &Some("de".to_owned()), 
        &Some("channel".to_owned())).unwrap();

    assert_eq!(
        path, 
        Path::new("/path/subs.srt").to_path_buf()
    )
}

#[test]
#[should_panic(expected = "File must end with an srt extension!")]
fn should_fail_if_file_path_does_not_have_srt_extension() {
    build_subtitle_path(
        Some(PathBuf::from_str("/path/subs.foo").unwrap()),
        Some(PathBuf::from_str("/arg_path").unwrap()),
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &Some("de".to_owned()), 
        &Some("channel".to_owned())).unwrap();
}

#[test]
#[should_panic(expected = "File must end with an srt extension!")]
fn should_fail_if_file_path_does_not_have_extension() {
    build_subtitle_path(
        Some(PathBuf::from_str("/path/subs").unwrap()),
        Some(PathBuf::from_str("/arg_path").unwrap()),
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &Some("de".to_owned()), 
        &Some("channel".to_owned())).unwrap();
}



#[test]
fn should_add_channel_and_file_name_if_path_is_directory() {
    let path = build_subtitle_path(
        None,
        Some(PathBuf::from_str("/arg_path").unwrap()), 
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &Some("de".to_owned()), 
        &Some("channel".to_owned())).unwrap();

    assert_eq!(
        path, 
        Path::new("/arg_path/channel/title.de.srt").to_path_buf()
    )
}

#[test]
fn should_exclude_lang_if_missing() {
    let path = build_subtitle_path(
        None,
        Some(PathBuf::from_str("/arg_path").unwrap()), 
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &None, 
        &Some("channel".to_owned())).unwrap();

    assert_eq!(
        path, 
        Path::new("/arg_path/channel/title.srt").to_path_buf()
    )
}

#[test]
fn should_exclude_channel_if_missing() {
    let path = build_subtitle_path(
        None,
        Some(PathBuf::from_str("/arg_path").unwrap()), 
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &Some("de".to_owned()), 
        &None).unwrap();

    assert_eq!(
        path, 
        Path::new("/arg_path/title.de.srt").to_path_buf()
    )
}

#[test]
fn should_use_config_path_if_arg_path_missing() {
    let path = build_subtitle_path(
        None,
        None, 
        Some(PathBuf::from_str("/config_path").unwrap()), 
        "title", 
        &Some("de".to_owned()), 
        &Some("channel".to_owned())).unwrap();

    assert_eq!(
        path, 
        Path::new("/config_path/channel/title.de.srt").to_path_buf()
    )
}