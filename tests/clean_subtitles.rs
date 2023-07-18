mod common;

use lang_tools::subtitle::clean::clean_subtitles;

use crate::common::subtitles;

#[test]
fn remove_opening_and_closing_html_elements() {
    let mut subtitles = subtitles(vec![
        "<i>Bin doch zufrieden.</i>\n<i>Mir geht’s doch gut.</i>",
    ]);

    clean_subtitles(&mut subtitles);

    assert_eq!(subtitles[0].text, "Bin doch zufrieden.\nMir geht’s doch gut.")
}

#[test]
fn remove_trailing_spaces() {
    let mut subtitles = subtitles(vec!["Ich erinnere mich nur noch an den Rauch   "]);

    clean_subtitles(&mut subtitles);

    assert_eq!(subtitles[0].text, "Ich erinnere mich nur noch an den Rauch")
}

#[test]
fn remove_leading_spaces() {
    let mut subtitles = subtitles(vec!["     Ich erinnere mich nur noch an den Rauch"]);

    clean_subtitles(&mut subtitles);

    assert_eq!(subtitles[0].text, "Ich erinnere mich nur noch an den Rauch")
}

#[test]
fn remove_multiple_inner_spaces() {
    let mut subtitles = subtitles(vec!["Ich erinnere    mich  nur noch an den Rauch"]);

    clean_subtitles(&mut subtitles);

    assert_eq!(subtitles[0].text, "Ich erinnere mich nur noch an den Rauch")
}

#[test]
fn remove_leading_space_second_line() {
    let mut subtitles = subtitles(vec![
        "Der vielleicht glücklichste Tag\n der Deutschen: der 9. November '89.",
    ]);

    clean_subtitles(&mut subtitles);

    assert_eq!(
        subtitles[0].text,
        "Der vielleicht glücklichste Tag\nder Deutschen: der 9. November '89."
    )
}

#[test]
fn remove_space_carriage_return_space() {
    let mut subtitles = subtitles(vec![
        "Der A350 macht unglaublich viel \r Spaß dieses Flugzeug zu fliegen.",
    ]);

    clean_subtitles(&mut subtitles);

    assert_eq!(
        subtitles[0].text,
        "Der A350 macht unglaublich viel Spaß dieses Flugzeug zu fliegen."
    )
}

#[test]
fn adjust_subtitle_ending_hyphen() {
    let mut subtitles = subtitles(vec![
        "Und die haben vielleicht mal für Y-",
        "Kollektiv irgendwas gedreht.",
    ]);

    clean_subtitles(&mut subtitles);

    assert_eq!(subtitles[0].text, "Und die haben vielleicht mal für Y-Kollektiv");
    assert_eq!(subtitles[1].text, "irgendwas gedreht.");
}

#[test]
fn adjust_hyphen_newline() {
    let mut subtitles = subtitles(vec![
        "dass 70% der Insel und des um-\nliegenden Archipels zerstört wurden."
    ]);

    clean_subtitles(&mut subtitles);

    assert_eq!(subtitles[0].text, "dass 70% der Insel und des umliegenden\nArchipels zerstört wurden.");
}
