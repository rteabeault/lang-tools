use clap::{command, Args, Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use url::Url;

#[derive(Parser)]
#[command(author, version, about, styles=get_styles())]
#[command(propagate_version = true)]
/// A set of tools to help in language learning.
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Gives subtitle information about a video accessible via yt-dlp.
    ///
    /// Given a url that is accessible via yt-dlp this command will
    /// display all the available languages and formats for
    /// subtitles.
    #[command(verbatim_doc_comment)]
    YtInfo(YtVideoArgs),

    /// Downloads subtitles from a video accessible via yt-dlp
    #[command(verbatim_doc_comment)]
    YtDownload(YtDownloadArgs),

    /// Aids in the translation of subtitles from a video accessbile via yt-dlp.
    ///
    /// Given a url that is accessible via yt-dlp this command does the following
    /// - Prompt the user for a subtitle language and format. This can be skipped
    ///   by passing the lang and format as an arg.
    /// - Downloads the subtitles and converts to SRT format if needed. It can
    ///   optionally save this subtitles locally.
    /// - Extracts the subtitles into sentences doing its best to place a each
    ///   sentence on its own line. These sentences are then placed into the
    ///   paste buffer.
    /// - You can then place these subtitles into a tool like Deepl to get translations.
    /// - Copy these translations and paste them into the editor that is opened.
    /// - After pressing enter these translaitons are then used to create target
    ///   language subtitles. These subtitles are then saved locally.
    #[command(verbatim_doc_comment)]
    YtTranslate(VideoTranslateArgs),

    /// Aids in the translation of SRT file locally available.
    ///
    /// Given a path of a valid SRT file
    /// - Extracts the subtitles into sentences doing its best to place a each
    ///   sentence on its own line. These sentences are then placed into the
    ///   paste buffer.
    /// - You can then place these subtitles into a tool like Deepl to get translations.
    /// - Copy these translations and paste them into the editor that is opened.
    /// - After pressing enter these translaitons are then used to create target
    ///   language subtitles. These subtitles are then saved locally.
    #[command(verbatim_doc_comment)]
    SRTTranslate(SRTTranslateArgs),

    /// Aids in the translation of text.
    #[command(verbatim_doc_comment)]
    TextTranslate(TextTranslateArgs),

    /// Aids in the translation of epub.
    /// 
    /// Creates a markdown file for each section with a source/target table.
    /// Creates a TOC file.
    ///
    /// lang-tools book translate "~/Dropbox/German/Books/Der Astronaut.epub" --output-path "~/Dropbox/German/Books"
    ///
    /// "~/Dropbox/German/Books/Der Astronaut/Der Astronaut.md" TOC
    /// "~/Dropbox/German/Books/Der Astronaut/Der Astronaut-1.md"
    /// "~/Dropbox/German/Books/Der Astronaut/Der Astronaut-2.md"
    ///
    /// base_path: ~/Dropbox/German/Books/
    /// book_path: ~/Dropbox/German/Books/Der Astronaut/
    /// section_path: ~/Dropbox/German/Books/Der Astronaut/Der Astronaut-1.md
    #[command(verbatim_doc_comment)]
    EpubTranslate(EpubTranslateArgs),
}

#[derive(Args, Debug)]
pub struct VideoTranslateArgs {
    #[command(flatten)]
    pub download_args: YtDownloadArgs,

    /// The language of the translated subtitles.
    ///
    /// <target_lang> can be defaulted with field <subtitle_target_lang>
    /// in ~/.config/lang-tools/config.yaml
    #[arg(long, requires = "format")]
    #[arg(verbatim_doc_comment)]
    pub target_lang: Option<String>,

    /// The directory path where target subtitles are written.
    ///
    /// The final file path is built as follows
    ///     
    ///    <output_path>/<yt_channel>/<title>.<target_lang>.srt
    ///
    /// Where yt_channel is the name of the video's channel or will be ommited if
    /// yt-dlp returns no channel for the video.
    ///
    /// output_path can be defaulted with field subtitle_output_path in ~/.config/lang-tools/config.yaml
    ///
    /// If neither --target_path or --target_file is provided then the target subtitle
    /// file will be written to
    ///
    ///     ./<title>.<target_lang>.srt
    ///
    /// <target_lang> will be set to "target" if it is not provided.
    #[arg(long, value_parser = PathBuf::from_str)]
    #[arg(verbatim_doc_comment)]
    pub target_path: Option<PathBuf>,

    /// The file where the translated subtitles will be written.
    ///
    /// Can not be used with --target-path
    ///
    /// If neither --target_path --target_file is provided then the target subtitle
    /// file will be written to
    ///
    ///     ./<title>.<target_lang>.srt
    #[arg(long, value_parser = PathBuf::from_str)]
    pub target_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct SRTTranslateArgs {
    #[arg(value_parser = PathBuf::from_str)]
    #[arg(verbatim_doc_comment)]
    /// The SRT file to be translated.
    ///
    /// The output file will be written to the same directory as source_path
    /// with the file name of
    ///
    /// <source_path_file_name>.<target_lang>.srt
    ///
    /// <target_lang> will be set to "target" if it is not provided.
    pub source_path: PathBuf,

    /// The language of the target subtitles.
    ///
    /// <target_lang> can be defaulted with field <subtitle_target_lang>
    /// in ~/.config/lang-tools/config.yaml
    #[arg(long, requires = "format")]
    #[arg(verbatim_doc_comment)]
    pub target_lang: Option<String>,
}

#[derive(Args, Debug)]
pub struct YtVideoArgs {
    /// The URL of the video.
    #[arg(value_parser = Url::from_str)]
    pub url: Url,
}

#[derive(Args, Debug)]
pub struct YtDownloadArgs {

    #[command(flatten)]
    pub video_args: YtVideoArgs,

    /// The format of the source subtitles.
    ///
    /// Requires --lang
    ///
    /// If not provided then a prompt will allow you to pick from a list of
    /// available languages and formats.
    ///
    /// If you want to see the available formats then run the subtitle-info command.
    #[arg[long, requires="lang"]]
    #[arg(verbatim_doc_comment)]
    pub format: Option<String>,

    /// The language of the source subtitles.
    ///
    /// Requires --format
    ///
    /// If not provided then a prompt will allow you to pick from a list of
    /// available languages and formats.
    ///
    /// If you want to see the available formats then run the subtitle-info command.
    #[arg(long, requires = "format")]
    #[arg(verbatim_doc_comment)]
    pub lang: Option<String>,

    /// The directory path where source subtitles are written.
    ///
    /// The final file path is built as follows
    ///     
    ///    <source_path>/<yt_channel>/<title>.<lang>.srt
    ///
    /// Where yt_channel is the name of the video's channel or will be ommited if
    /// yt-dlp returns no channel for the video.
    ///
    /// source_path can be defaulted with field subtitle_source_path in ~/.config/lang-tools/config.yaml
    ///
    /// If neither --source_path or --source_file is provided then the source subtitle
    /// file will be written to
    ///
    ///     ./<title>.<lang>.srt
    #[arg(long, value_parser = PathBuf::from_str)]
    #[arg(verbatim_doc_comment)]
    pub source_path: Option<PathBuf>,

    /// The file where the source subtitles will be written.
    ///
    /// Can not be used with --output_path.
    #[arg(long, value_parser = PathBuf::from_str)]
    pub source_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct TextTranslateArgs {
    #[arg(long, short)]
    pub source_file: Option<PathBuf>,
    
    #[arg(long, short)]
    pub target_file: Option<PathBuf>,

    #[arg(long, short)]
    pub output_file: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct EpubTranslateArgs {
    pub input_file: PathBuf,

    /// The base path where translations will be written.
    #[arg(long, short)]
    pub output_path: Option<PathBuf>,

    /// Output as a markdown table with source text in the left column and target in the right
    #[arg(long, short = 't', default_value_t = true)]
    pub output_markdown_table: bool,
    
    /// Output as alternating paragraphs of source lang followed by its translation
    #[arg(long, short = 'a')]
    pub output_alternating_langs: bool,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub subtitle_target_path: Option<PathBuf>,
    pub subtitle_target_lang: Option<String>,
    pub subtitle_source_path: Option<PathBuf>,
    pub books_target_path: Option<PathBuf>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            subtitle_target_path: Default::default(),
            subtitle_target_lang: Default::default(),
            subtitle_source_path: Default::default(),
            books_target_path: Default::default(),
        }
    }
}

fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))),
        )
        .literal(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
}
