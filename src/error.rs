use std::str::FromStr;

use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum DelugeApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("TorrentResponse JSON doesn't satisfy response schema")]
    Json,
    #[error(transparent)]
    Deluge(#[from] DelugeError),
    #[error(transparent)]
    TryInto(#[from] std::num::TryFromIntError),
}

#[derive(Error, Debug)]
pub enum DelugeError {
    #[error("Tried to add torrent already in session (id: {0})")]
    DuplicateTorrent(String),

    #[error("{0}")]
    Other(String),
}

impl FromStr for DelugeError {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r#"Torrent already in session \((?P<id>[[:alnum:]]+)\)."#).unwrap();
        }
        let out = match RE.captures(s) {
            Some(x) => Self::DuplicateTorrent(x.name("id").unwrap().as_str().to_owned()),
            None => Self::Other(s.to_owned()),
        };

        Ok(out)
    }
}

#[cfg(test)]
mod test {
    use crate::error::DelugeError;

    #[test]
    fn parse_blank_string_error() {
        let test_str = "";
        assert!(match test_str.parse::<DelugeError>().unwrap() {
            DelugeError::Other(info) => {
                info == test_str
            }
            _ => false,
        });
    }
    #[test]
    fn parse_duplicate_torrent_error() {
        let test_id = "asdf";
        let test_str = format!("Torrent already in session ({test_id}).");
        assert!(match test_str.parse::<DelugeError>().unwrap() {
            DelugeError::DuplicateTorrent(id) => {
                id == test_id
            }
            _ => false,
        });
    }
}
