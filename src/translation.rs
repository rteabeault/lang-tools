use anyhow::anyhow;
use serde::{Deserialize, Serialize};
use tabled::Tabled;

use crate::proportional::ProportionalIter;

#[derive(Debug, Serialize, Deserialize, Clone, Tabled)]
pub struct Translation {
    #[tabled(rename = "Source")]
    pub source_text: String,

    #[tabled(rename = "Translated")]
    pub target_text: String,
}

impl Translation {
    pub fn new(source_text: &str, target_text: &str) -> Self {
        return Translation {
            source_text: source_text.to_owned(),
            target_text: target_text.to_owned(),
        };
    }

    pub fn from_source_and_target(
        source: &str,
        target: &str,
    ) -> Result<Vec<Translation>, anyhow::Error> {
        let source: Vec<&str> = source.lines().collect();
        let target: Vec<&str> = target.lines().collect();

        if source.len() != target.len() {
            return Err(anyhow!("There are {} lines in source and {} lines in target. Number of lines must be of equal length", source.len(), target.len()));
        } else {
            return Ok(source
                .into_iter()
                .zip(target.into_iter())
                .map(|parts| Translation::new(parts.0, parts.1))
                .collect());
        }
    }
}

pub trait VecExt {
    fn proportioned(&self) -> Box<dyn Iterator<Item = (&str, Vec<&str>)> + '_>;
}

impl VecExt for Vec<Translation> {
    fn proportioned(&self) -> Box<dyn Iterator<Item = (&str, Vec<&str>)> + '_> {
        Box::new(
            self.iter().flat_map(|t| {
                ProportionalIter::new(t.source_text.as_str(), t.target_text.as_str())
            }),
        )
    }
}

impl VecExt for [Translation] {
    fn proportioned(&self) -> Box<dyn Iterator<Item = (&str, Vec<&str>)> + '_> {
        Box::new(
            self.iter().flat_map(|t| {
                ProportionalIter::new(t.source_text.as_str(), t.target_text.as_str())
            }),
        )
    }
}
