use crate::cli::Config;
use crate::cli::SRTTranslateArgs;
use crate::common::prompt_for_clipboard_read;
use crate::subtitle::SUB_TRANSLATE_MSG;
use crate::subtitle::print_translated_subtitles_written_to;
use lang_tools::clipboard::set_clipboard;
use lang_tools::subtitle::clean::clean_subtitles;
use lang_tools::subtitle::path::build_subtitle_path_from_path;
use lang_tools::subtitle::path::load_from_path;
use lang_tools::subtitle::write::write_subtitles;
use lang_tools::subtitle::{extract::extract_text, translation::translated_subtitles};

pub fn exec(args: SRTTranslateArgs, config: Config) -> Result<(), anyhow::Error> {
    let mut subtitles = load_from_path(&args.source_path)?;

    clean_subtitles(&mut subtitles);

    let subtitle_text = extract_text(&subtitles);

    set_clipboard(&subtitle_text)?;

    let translated_text = prompt_for_clipboard_read(SUB_TRANSLATE_MSG)?;

    let translated_subs = translated_subtitles(&subtitles, &subtitle_text, &translated_text)?;

    let path = build_subtitle_path_from_path(
        args.source_path,
        args.target_lang,
        config.subtitle_target_lang,
    );

    let path = write_subtitles(&path, &translated_subs)?;

    print_translated_subtitles_written_to(&path);

    Ok(())
}
