use crate::{
    book::prompt_book_translation,
    cli::{EpubTranslateArgs, Config}, common::{print_info, print_bracketed_info}
};
use epub::doc::EpubDoc;
use lang_tools::{book::{book_path, epub::get_book_title, BookSection}, path::first_path_or_current_dir};

pub fn exec(args: EpubTranslateArgs, config: Config) -> Result<(), anyhow::Error> {
    let epub = EpubDoc::new(args.input_file)?;

    let base_output_path =
        first_path_or_current_dir(vec![args.output_path, config.books_target_path])?;

    let book_title = get_book_title(&epub)?;

    print_bracketed_info("Loaded epub", &book_title);

    let book_path = book_path(&base_output_path, &book_title);

    print_bracketed_info("Writing translations to", &book_path.to_string_lossy());

    let mut sections: Vec<BookSection> = BookSection::from_epub(epub);

    prompt_book_translation(&book_path, &mut sections)?;

    print_info("- Book translation session complete!");

    Ok(())
}
