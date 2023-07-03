use anyhow::{Context, Error};
use fancy_regex::Regex;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use srtlib::Subtitles;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use url::Url;
use youtube_dl::{SingleVideo, YoutubeDl, YoutubeDlOutput};

lazy_static! {
    static ref LOOKS_LIKE_LANGUAGE: Regex = Regex::new(r"^(?P<lang>[a-zA-Z]{2})-.*$").unwrap();
}

pub fn info(url: &Url) -> Result<VideoInfo, Error> {
    match YoutubeDl::new(url.to_owned())
        .socket_timeout("15")
        .run()
        .context(format!("Failed to fetch info for video at [{}]", url.to_owned()))? {

        YoutubeDlOutput::SingleVideo(v) => Ok(VideoInfo::try_from(*v)?),
        YoutubeDlOutput::Playlist(_p) => Err(anyhow::Error::msg("Playlists not supported")),
    }
}

pub fn download(url: &Url) -> Result<Subtitles, anyhow::Error> {
    let subtitles = reqwest::blocking::get(url.to_owned())?
        .text()
        .context(format!("Failed to download {}", url))?;

    let subtitles = convert_to_srt(subtitles.as_bytes())?;

    return Subtitles::parse_from_str(subtitles).context("Failed to parse subtitles file.");
}

fn convert_to_srt(source: &[u8]) -> Result<String, anyhow::Error> {
    let ffmpeg = Command::new("ffmpeg")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("-loglevel")
        .arg("quiet")
        .arg("-i")
        .arg("pipe:0")
        .arg("-f")
        .arg("srt")
        .arg("pipe:1")
        .spawn()
        .context("Unable to convert subtitles to SRT format. Ensure that ffmpeg is installed and on the PATH")?;

    ffmpeg.stdin.unwrap().write_all(source)?;

    let mut s = String::new();
    ffmpeg.stdout.unwrap().read_to_string(&mut s)?;

    Ok(s)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubtitleChoice {
    pub lang: String,
    pub format: String,
    pub location: Url,
}

impl SubtitleChoice {
    pub fn new(lang: String, format: String, location: Url) -> Self {
        SubtitleChoice {
            lang,
            format,
            location,
        }
    }
}

impl std::fmt::Display for SubtitleChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Language: {}, Format: {}", self.lang, self.format)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VideoInfo {
    pub name: String,
    pub channel: Option<String>,
    pub choices: Vec<SubtitleChoice>,
}

impl VideoInfo {
    pub fn new(name: String, channel: Option<String>, choices: Vec<SubtitleChoice>) -> Self {
        VideoInfo {
            name,
            channel,
            choices,
        }
    }
}

impl TryFrom<SingleVideo> for VideoInfo {
    type Error = anyhow::Error;

    fn try_from(video: SingleVideo) -> Result<Self, Self::Error> {
        let subs = video.subtitles.unwrap_or_default();

        // TODO clean this up.
        let choices: Result<Vec<SubtitleChoice>, Self::Error> =
            subs.into_iter().try_fold(Vec::new(), |mut acc, (lang, s)| {
                for choice in s.unwrap_or_default() {
                    if let (Some(subtitle_format), Some(url)) = (choice.ext, choice.url) {
                        if subtitle_format == "vtt" || subtitle_format == "srt" {
                            let location = Url::parse(&url)?;
                            let choice =
                                SubtitleChoice::new(lang.to_owned(), subtitle_format, location);
                            acc.push(choice);
                        }
                    }
                }

                Ok(acc)
            });

        Ok(VideoInfo::new(video.title, video.channel, choices?))
    }
}
