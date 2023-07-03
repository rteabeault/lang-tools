use crate::{
    cli::{Config, TextTranslateArgs},
    common::{print_bracketed_info, print_info, prompt_for_clipboard_read},
};

use lang_tools::{
    clipboard::set_clipboard,
    file::{read_from_file, write_to_file},
    translation::Translation,
};
use tabled::{settings::Style, Table};

pub fn exec(args: TextTranslateArgs, _config: Config) -> Result<(), anyhow::Error> {
    let content = match args.source_file {
        Some(file) => read_from_file(file)?,
        None => prompt_for_clipboard_read(
            "Copy the text into past buffer and press 'y'. Or press 'n' to quit",
        )?,
    };

    let translated = match args.target_file {
        Some(file) => read_from_file(file)?,
        None => prompt_for_clipboard_read(
            "Copy the translated text into past buffer and press 'y'. Or press 'n' to quit",
        )?,
    };

    let translations = Translation::from_source_and_target(content.as_str(), translated.as_str())?;

    let table = Table::new(translations).with(Style::markdown()).to_string();

    match args.output_file {
        Some(output_file) => {
            print_bracketed_info("Writing translations to", &output_file.to_string_lossy());
            write_to_file(&output_file, &table)?
        }
        None => {
            print_info("Plancing translations into clipboard.");
            set_clipboard(&table)?
        }
    }

    Ok(())
}
