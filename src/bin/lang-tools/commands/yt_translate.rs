use crate::cli::{Config, VideoTranslateArgs};
use crate::common::prompt_for_clipboard_read;
use crate::subtitle::{
    fetch_video_info, print_translated_subtitles_written_to, selecte_subtitle, SUB_TRANSLATE_MSG,
};
use anyhow::Result;
use lang_tools::clipboard::set_clipboard;
use lang_tools::subtitle::clean::clean_subtitles;
use lang_tools::subtitle::extract::extract_text;
use lang_tools::subtitle::path::build_subtitle_path;
use lang_tools::subtitle::translation::translated_subtitles;
use lang_tools::subtitle::write::write_subtitles;
use lang_tools::youtube;

pub fn exec(args: VideoTranslateArgs, config: Config) -> Result<(), anyhow::Error> {
    let info = fetch_video_info(&args.download_args.video_args.url)?;

    let choice = selecte_subtitle(
        args.download_args.lang,
        args.download_args.format,
        &info.choices,
    )?;

    let mut subtitles = youtube::download(&choice.location)?;

    clean_subtitles(&mut subtitles);

    let source_path = build_subtitle_path(
        args.download_args.source_file,
        args.download_args.source_path,
        config.subtitle_source_path,
        &info.name,
        &Some(choice.lang),
        &info.channel,
    )?;

    let source_path = write_subtitles(&source_path, &subtitles)?;
    println!("Source subtitles saved to [{:?}]", source_path);

    let subtitle_text = extract_text(&subtitles);

    set_clipboard(&subtitle_text)?;

    let translated_text = prompt_for_clipboard_read(SUB_TRANSLATE_MSG)?;

    let target_subs = translated_subtitles(&subtitles, &subtitle_text, &translated_text)?;

    let target_path = build_subtitle_path(
        args.target_file,
        args.target_path,
        config.subtitle_target_path,
        &info.name,
        &args.target_lang.or(config.subtitle_target_lang),
        &info.channel,
    )?;

    let target_path = write_subtitles(&target_path, &target_subs)?;

    print_translated_subtitles_written_to(&target_path);

    Ok(())
}
