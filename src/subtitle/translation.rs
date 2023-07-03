use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;
use srtlib::{Subtitle, Subtitles};

use crate::{translation::{Translation, VecExt}};

use super::ext::SubtitleExt;

lazy_static!{ 
    // Regex that matches 2 or more spaces.
    static ref MULTI_SPACE_RE: Regex = Regex::new("[ ]{2,}").unwrap();
}

pub fn translated_subtitles(source_subtitles: &Subtitles, subtitle_text: &str, translated_text: &str) -> Result<Subtitles, anyhow::Error> {
    let translations = 
        Translation::from_source_and_target(subtitle_text, translated_text)?;

    return align(source_subtitles, &translations)
}

pub fn align(
    subtitles: &Subtitles,
    translations: &[Translation]
) -> Result<Subtitles, anyhow::Error> {
    let subtitles_iter = subtitles.into_iter();
    let mut word_iter = translations.proportioned();

    let translated_subtitles = subtitles_iter.map(|subtitle| {
        // trace!("Aligning subtitle [{}] with text: {}", subtitle.num, subtitle.text);
        let subtitle_tokens = subtitle.words_and_spaces();

        let translated_text: String = subtitle_tokens.map(|subtitle_token| {
            // trace!("Current subtitle token [{}]", subtitle_token);

            if subtitle_token.trim().is_empty() {
                // trace!("Empty. Adding value [{}]", subtitle_token);
                subtitle_token.to_owned()
            } else if let Some((source_word, target_words)) = word_iter.next() {
                if subtitle_token == source_word {
                    let target = target_words.iter().join(" ");
                    // trace!("Adding translation: [{}]", target);
                    target
                } else {
                    panic!("The subtitle word [{}] did not equal the next source word [{}] \
                    from the translation. This is unexpected and should be considered a bug.",
                    subtitle_token, source_word);
                }
            } else {
                panic!("The subtitle word is not empty. There are no more translated words.\
                This should be considered a bug")
            }
        }).collect();

        // There is a chance that a space is followed by a translation that has no words. In that
        // case we get two spaces. Let's replace those.
        let translated_text = MULTI_SPACE_RE.replace(&translated_text, " ");

        // There is a chance that there is a space token followed by the last translation which
        // is empty. In that case we get an extra space at the end. Get rid of it.
        let translated_text = translated_text.trim();

        Subtitle::new(subtitle.num, subtitle.start_time, subtitle.end_time, translated_text.to_string())
    }).collect();

    Ok(Subtitles::new_from_vec(translated_subtitles))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::subtitle::util::test_util::subtitles;

    #[test]
    fn test_single_subtitle_with_equal_length_translation() {
        let translations =
            vec![Translation::new("Source text", "Target text")];
        let subs = subtitles(vec!["Source text"]);

        let aligned = align(&subs, &translations).unwrap();

        assert_eq!(aligned, subtitles(vec!["Target text"]))
    }

    #[test]
    fn test_two_subtitles_one_translation() {
        let translations = vec![
                Translation::new(
                    "Es geht um Mountainbiker und die Frage: Wer darf hier wie den Wald \
                    nutzen?",
                    "It's about mountain bikers and the question: Who is allowed to \
                    use the forest here and how?")
            ];

        let subs = subtitles(vec![
            "Es geht um Mountainbiker\nund die Frage:",
            "Wer darf hier wie den Wald nutzen?"
        ]);

        let aligned = align(&subs, &translations).unwrap();

        assert_eq!(aligned, subtitles(vec![
            "It's about mountain bikers\nand the question: Who",
            "is allowed to use the forest here and how?"]))
    }

    #[test]
    fn test_weird_space() {
        //     "source_text": "Wir haben euch gefragt, hat von euch schon mal jemand gecatfisht?",
        //     "target_text": "We asked you, has anyone of you ever catfished?"

        let translations =
            vec![Translation::new(
                "Wir haben euch gefragt, hat von euch schon mal jemand gecatfisht?",
                "We asked you, has anyone of you ever catfished?")];

        let subs = subtitles(vec!["Wir haben euch gefragt, hat von euch schon mal jemand gecatfisht?"]);
        let aligned = align(&subs, &translations).unwrap();

        assert_eq!(aligned, subtitles(vec!["We asked you, has anyone of you ever catfished?"]));
    }
}