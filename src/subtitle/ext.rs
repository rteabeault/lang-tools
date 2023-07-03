use fancy_regex::Regex;
use lazy_static::lazy_static;
use srtlib::Subtitle;

lazy_static! {
    static ref SPACES_RE: Regex = Regex::new(r"(\S+|\s+)").expect("Invalid regex");
}

pub trait SubtitleExt<'a> {
    /// Returns an iterator over all of the words and spaces/newlines of the
    /// subtitle text.
    /// For example, "  This is  some text. " would return an iterator with the items
    /// "  ", "This", " ", "is", "  ", "some", " ", "text."
    fn words_and_spaces(&'a self) -> Box<dyn Iterator<Item=&'a str> + 'a>;
}

// TODO fix unwrap
impl<'a> SubtitleExt<'a> for Subtitle {
    fn words_and_spaces(&'a self) -> Box<dyn Iterator<Item=&'a str> + 'a> {
        Box::new(SPACES_RE.captures_iter(&self.text)
            .map(|c| c.unwrap().get(1).map_or("", |m| m.as_str())))
    }
}