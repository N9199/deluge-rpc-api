use lazy_static::lazy_static;
use regex::{Regex, RegexSet};
use thiserror::Error;

use crate::torrent_stuff::ErrorValue;

#[derive(Error, Debug)]
pub enum DelugeApiError {
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("TorrentResponse JSON doesn't satisfy response schema")]
    Json,
    #[error("Incorrectly empty result")]
    EmptyResult,
    #[error(transparent)]
    Deluge(#[from] DelugeError),
    #[error(transparent)]
    TryInto(#[from] std::num::TryFromIntError),
    #[error("Header values are not ASCII complaint")]
    IncorrectHeaderFormat,
}

#[derive(Error, Debug)]
pub enum DelugeError {
    #[error("Tried to add torrent already in session (id: {0})")]
    DuplicateTorrent(String),

    #[error("{0}")]
    Other(String),
}

impl From<ErrorValue> for DelugeError {
    fn from(e: ErrorValue) -> Self {
        e.message.into()
    }
}
const N: usize = 1;
const ERROR_REGEX: [&str; N] = [r#"Torrent already in session \((?P<id>[[:alnum:]]+)\)."#];
lazy_static! {
    static ref RE: RegexSet = RegexSet::new(&ERROR_REGEX).unwrap();
}
lazy_static! {
    static ref RES: [Regex; N] = ERROR_REGEX.map(|x| Regex::new(x).unwrap());
}
impl From<String> for DelugeError {
    fn from(val: String) -> Self {
        let matches = RE.matches(&val);
        if matches.matched_any() {
            let m = unsafe { matches.into_iter().next().unwrap_unchecked() };
            match m {
                0 => Self::DuplicateTorrent(unsafe {
                    RES[0]
                        .captures(&val)
                        .unwrap_unchecked()
                        .name("id")
                        .unwrap_unchecked()
                        .as_str()
                        .to_owned()
                }),
                _ => Self::Other(val),
            }
        } else {
            Self::Other(val)
        }
    }
}

impl From<String> for DelugeApiError {
    fn from(val: String) -> Self {
        let temp: DelugeError = val.into();
        temp.into()
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::pedantic)]
    use crate::DelugeError;

    #[test]
    fn parse_blank_string_error() {
        let test_str = String::new();
        assert!(match test_str.clone().into() {
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
        assert!(match test_str.into() {
            DelugeError::DuplicateTorrent(id) => {
                id == test_id
            }
            _ => false,
        });
    }
}
