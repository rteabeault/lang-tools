use std::{path::{PathBuf, Path}, fs::File, io::Read};

use anyhow::Context;
use clap::Parser;
use cli::{Commands, Cli, Config};
use console::style;
use lazy_static::lazy_static;

pub mod commands;
pub mod common;
pub mod cli;
pub mod book;
pub mod subtitle;

lazy_static! {
    pub static ref CONFIG_PATH: String = shellexpand::tilde("~/.config/lang-tools/config.yaml").to_string();
}

fn main() {
    if let Err(err) = try_main() {
        eprintln!("{}", style(format!("Exiting: {:#}", err)).red());
        std::process::exit(1);
    }
}

fn try_main() -> Result<(), anyhow::Error> {
    
    let cli = Cli::parse();

    let config = load_config(Path::new(&*CONFIG_PATH).to_path_buf())?;

    println!("{}", style("Welcome to lang-tools!").bold().cyan().underlined());

    match cli.command {
        Commands::SRTTranslate(args) => commands::srt_translate::exec(args, config),
        Commands::YtTranslate(args) => commands::yt_translate::exec(args, config),
        Commands::YtInfo(args) => commands::yt_info::exec(args, config),
        Commands::YtDownload(args) => commands::yt_download::exec(args, config),
        Commands::TextTranslate(args) => commands::text_translate::exec(args, config),
        Commands::EpubTranslate(args) => commands::epub_translate::exec(args, config)
    }
}

fn load_config(path: PathBuf) -> Result<Config, anyhow::Error> {
    if path.exists() {
        let mut f = File::open(&path).unwrap();
    
        let mut config = String::new();
        f.read_to_string(&mut config).unwrap();
    
        return serde_yaml::from_str(&config.as_str())
            .context(format!("Failed loading config from {}", path.into_os_string().into_string().unwrap()))
    } else {
        return Ok(Config::default())
    }
}

