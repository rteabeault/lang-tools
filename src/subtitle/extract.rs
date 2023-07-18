use fancy_regex::Regex;
use itertools::Itertools;
use lazy_static::lazy_static;
use srtlib::Subtitles;

lazy_static! {
    // Regex that matches a line that ends with an alphanumeric, comma, collon, or semi-colon.
    static ref JOIN_RE: Regex = Regex::new(r"([A-Za-z0-9,;:])\n").unwrap();

    // Regex that matches an initial.
    static ref INITIAL_RE: Regex = Regex::new(r"(\s[A-Z0-9][.])\n").unwrap();

    // A line that ends with a hyphen but has a space before it.
    static ref END_HYPHEN: Regex = Regex::new(r"\s-\n").unwrap();
}

pub fn extract_text(subtitles: &Subtitles) -> String {
    let x = subtitles
        .into_iter()
        .map(|s| s.text.lines().map(|l| l.trim()).join("\n"))
        .join("\n");

    return join_sentences(x);
}

fn join_sentences(text: String) -> String {
    let text = JOIN_RE.replace_all(&text, "$1 ");
    let text = INITIAL_RE.replace_all(&text, "$1 ");
    let text = END_HYPHEN.replace_all(&text, " - ");
    return text.to_string();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::subtitle::util::test_util::subtitles;

    #[test]
    fn treat_single_uppercase_letter_followed_by_period_as_part_of_sentence() {
        let subtitles = subtitles(vec!["7 h später stirbt Susanne F.\nan ihren Verletzungen."]);

        let text = extract_text(&subtitles);

        assert_eq!(text, "7 h später stirbt Susanne F. an ihren Verletzungen.")
    }

    #[test]
    fn treat_line_ending_with_number_followed_by_period_as_part_of_sentence() {
        let subtitles = subtitles(vec!["Heute sieht sie diese Aufnahmen zum 1.\nMal."]);

        let text = extract_text(&subtitles);

        assert_eq!(text, "Heute sieht sie diese Aufnahmen zum 1. Mal.")
    }

    #[test]
    fn join_hyphen_at_end_of_line() {
        let subtitles = subtitles(vec![
            "Die Folgen - damals wie heute -\nein Problem für die jetzt 33-Jährige.",
        ]);

        let text = extract_text(&subtitles);

        assert_eq!(
            text,
            "Die Folgen - damals wie heute - ein Problem für die jetzt 33-Jährige."
        )
    }

    #[test]
    fn astericks_newline() {
        let subtitles = subtitles(vec![
            "Davon jedenfalls\ngeht Sigrid Kamisch aus.",
            "* Musik *",
            "Dann habe ich\neinen Freund angerufen, der bei der Polizei arbeitet.",
        ]);

        let text = extract_text(&subtitles);

        assert_eq!(text, "Davon jedenfalls geht Sigrid Kamisch aus.\n* Musik *\nDann habe ich einen Freund angerufen, der bei der Polizei arbeitet.")
    }
}
