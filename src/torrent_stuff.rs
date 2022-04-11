use std::fmt;

use reqwest::Url;
use serde::{Deserialize, Serialize};
use serde_json::Value;

mod torrent_options;
pub use torrent_options::*;

use crate::error::DelugeError;
pub enum TorrentStatus {}

pub struct TorrentTracker {
    pub url: Url,
    pub tier: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TorrentResponse {
    pub result: Value,
    id: usize,
    pub error: Option<ErrorValue>,
}

impl fmt::Display for TorrentResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value_type, value) = match &self.error {
            Some(error) => (
                "error",
                error.message.parse::<DelugeError>().unwrap().to_string(),
            ),
            None => ("result", self.result.to_string()),
        };
        write!(f, "TorrentResponse {{ {value_type}: {value} }}")
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ErrorValue {
    code: usize,
    pub message: String,
}
