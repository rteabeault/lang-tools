use anyhow::anyhow;
use console::Style;
use console::style;
use dialoguer::{theme::ColorfulTheme, Confirm};
use lang_tools::clipboard::get_clipboard;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref MSG_STYLE: Style = Style::new().cyan();
}

pub fn dialoguer_theme() -> ColorfulTheme {
    ColorfulTheme::default()
}

pub fn prompt_for_clipboard_read(msg: &str) -> Result<String, anyhow::Error> {
    if Confirm::new().with_prompt(msg).interact()? {
        get_clipboard()
    } else {
        Err(anyhow!("User cancelled reading from clipboard"))
    }
}

pub fn print_bracketed_info(msg: &str, bracket_msg: &str) {
    println!("{}", style(format!("- {} [{}]", msg, style(bracket_msg).underlined())).cyan());
}

pub fn print_info(msg: &str) {
    println!("- {}", style(msg).cyan())
}

pub fn print_error(msg: &str) {
    println!("- {}", style(msg).red())
}