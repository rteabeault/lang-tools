use crate::{
    cli::{Config, YtDownloadArgs},
    subtitle::{selecte_subtitle, fetch_video_info, print_subtitles_written_to},
};

use anyhow::Context;

use lang_tools::{
    subtitle::{clean::clean_subtitles, path::build_subtitle_path, write::write_subtitles},
    youtube,
};

pub fn exec(args: YtDownloadArgs, config: Config) -> Result<(), anyhow::Error> {
    
    let info = fetch_video_info(&args.video_args.url)?;

    let subtitle_choice = 
        selecte_subtitle(args.lang, args.format, &info.choices)
            .context("Exiting...")?;
    
    let mut subtitles = youtube::download(&subtitle_choice.location)?;

    clean_subtitles(&mut subtitles);

    let path = build_subtitle_path(
        args.source_file,
        args.source_path,
        config.subtitle_source_path,
        &info.name,
        &Some(subtitle_choice.lang),
        &info.channel,
    )?;

    let path = write_subtitles(&path, &subtitles)?;
    print_subtitles_written_to(&path);

    Ok(())
}
