use itertools::Itertools;
use srtlib::{Subtitles, Subtitle};

use fancy_regex::Regex;
use lazy_static::lazy_static;

lazy_static! {
    // Regex that matches an HTML tag.
    static ref HTML_TAG_RE: Regex = Regex::new("<.*?>").unwrap();

    // Regex that matches 2 or more spaces.
    static ref MULTI_SPACE_RE: Regex = Regex::new("[ ]{2,}").unwrap();

    // Regex that mathes a two spaces with a carriage return in between.
    static ref CARRIAGE_RETURN_RE: Regex = Regex::new(r"\s+\r\s+").unwrap();

    // A line that starts with a space
    static ref SPACES_AROUND_NEW_LINES: Regex = Regex::new(r"\s*\n\s*").unwrap();

    // A subtitle where the text ends abruptly with a hyphen
    // For example:
    // 581
    // 00:16:43,770 --> 00:16:45,360
    // Und die haben vielleicht mal für Y-
    static ref END_SUBTITLE_TEXT_WITH_HYPHEN: Regex = Regex::new(r"([A-Za-z])-$").unwrap();

    // A subtitle line where a line ends with a hyphenated word. 
    // 44
    // 00:02:19,320 --> 00:02:23,440
    // dass 70% der Insel und des um-
    // liegenden Archipels zerstört wurden.
    static ref MID_SUBTITLE_HYPHENATED_WORD: Regex = Regex::new(r"([A-Za-z])-\n(\w*)\s").unwrap();

    // A subtitle line that ends with one or more \h.
    // 6
    // 00:00:25,320 --> 00:00:31,080
    // Vielleicht sind wir früh dran und entstanden\h
    // quasi vor allem anderen Leben - das könnte\h\h
    static ref SLASH_H: Regex = Regex::new(r"\\h").unwrap();

}

pub fn clean_subtitles(subtitles: &mut Subtitles) {
    for subtitle in subtitles.into_iter() {
        subtitle.text = clean(&subtitle.text);
    }

    for (subtitle, subtitle_next) in subtitles.into_iter().tuples() {
        adjust_hyphen_ending_subtitle(subtitle, subtitle_next)
    }
}

fn clean(text: &str) -> String {
    let text = SLASH_H.replace_all(&text, "");
    let text = HTML_TAG_RE.replace_all(&text, "");
    let text = MULTI_SPACE_RE.replace_all(&text, " ");
    let text = CARRIAGE_RETURN_RE.replace_all(&text, " ");
    let text = SPACES_AROUND_NEW_LINES.replace_all(&text, "\n");
    let text = MID_SUBTITLE_HYPHENATED_WORD.replace_all(&text, "$1$2\n");
    return text.trim().to_owned();
}

/// Consider the subtitles
///
/// 581
/// 00:16:43,770 --> 00:16:45,360
/// Und die haben vielleicht mal für Y-
///
/// 582
/// 00:16:45,361 --> 00:16:46,949
/// Kollektiv irgendwas gedreht.
///
/// The word at the end of subtitle 581 is Y-Kollektiv but unfortunately
/// Kollektiv has been moved to the next subtitle. This can cause issues later
/// in the process when trying to align subtitles with translations. If we see
/// this pattern of a subtitle text ending with r"([A-Za-z])-\n" then lets pull
/// the first word of the next line up.
fn adjust_hyphen_ending_subtitle(subtitle: &mut Subtitle, subtitle_next: &mut Subtitle) {
    if END_SUBTITLE_TEXT_WITH_HYPHEN
        .is_match(&subtitle.text)
        .unwrap()
    {
        if let Some(idx) = subtitle_next.text.find(" ") {
            let word = subtitle_next.text[0..idx].to_owned();

            subtitle_next.text.replace_range(0..idx + 1, "");
            subtitle.text.push_str(word.trim());
        }
    }
}
