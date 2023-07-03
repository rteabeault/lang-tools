#[cfg(test)]
pub(crate) mod test_util {
    use srtlib::{Subtitle, Subtitles, Timestamp};

    pub fn subtitles(subtitles: Vec<&str>) -> Subtitles {
        let x = subtitles
            .into_iter()
            .enumerate()
            .map(|(index, text)| {
                // 00:00:03,240 --> 00:00:06,920
                Subtitle::new(
                    index + 1,
                    Timestamp::new(0, 0, 0, 0),
                    Timestamp::new(0, 0, 1, 0),
                    text.to_owned(),
                )
            })
            .collect();

        return srtlib::Subtitles::new_from_vec(x)
    }
}
