use crate::{
    book::prompt_book_translation,
    cli::{EpubTranslateArgs, Config}, common::{print_info, print_bracketed_info}
};
use epub::doc::EpubDoc;
use lang_tools::{book::{book_path, epub::get_book_title, toc_path, Chapter}, path::first_path_or_current_dir};

pub fn exec(args: EpubTranslateArgs, config: Config) -> Result<(), anyhow::Error> {
    let epub = EpubDoc::new(args.input_file)?;

    let base_output_path =
        first_path_or_current_dir(vec![args.output_path, config.books_target_path])?;

    let book_title = get_book_title(&epub)?;

    print_bracketed_info("Loaded epub", &book_title);

    let book_path = book_path(&base_output_path, &book_title);

    print_bracketed_info("Writing translations to", &book_path.to_string_lossy());

    let toc_path = toc_path(&book_path, &book_title);

    let chapters = Chapter::from_epub(epub, &book_title, &book_path);

    prompt_book_translation(&toc_path, &book_title, &chapters)?;

    print_info("- Book translation session complete!");

    Ok(())
}
