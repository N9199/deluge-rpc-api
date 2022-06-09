use std::fmt;

use camino::Utf8PathBuf;
use reqwest::Url;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

mod torrent_options;
pub use torrent_options::*;

use crate::error::{DelugeApiError, DelugeError};
#[derive(Debug, Deserialize, Serialize)]
pub enum TorrentStatus {} // TODO fill up see https://www.libtorrent.org/reference-Torrent_Status.html#torrent_status

#[derive(Debug)]
pub struct TorrentTracker {
    pub url: Url,
    pub tier: usize,
}

impl Serialize for TorrentTracker {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("TorrentTracker", 2)?;
        state.serialize_field("url", self.url.as_str())?;
        state.serialize_field("tier", &self.tier)?;
        state.end()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct TorrentResponse<V> {
    pub result: Option<V>,
    id: usize,
    pub error: Option<ErrorValue>,
}

impl<V> TorrentResponse<V> {
    pub(crate) fn into_result(self) -> Result<V, DelugeApiError> {
        if let Some(err) = self.error {
            Err(err.message.parse::<DelugeError>().unwrap().into())
        } else {
            self.result.ok_or(DelugeApiError::EmptyResult)
        }
    }
}

impl TorrentResponse<()> {
    pub(crate) fn into_empty_result(self) -> Result<(), DelugeApiError> {
        let out = self.into_result();
        match out {
            Ok(_) => Ok(()),
            Err(err) => match err {
                DelugeApiError::EmptyResult => Ok(()),
                _ => Err(err),
            },
        }
    }
}

impl<V> fmt::Display for TorrentResponse<V>
where
    V: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (value_type, value) = match &self.error {
            Some(error) => (
                "error",
                error.message.parse::<DelugeError>().unwrap().to_string(),
            ),
            None => ("result", format!("{:?}", self.result)),
        };
        write!(f, "TorrentResponse {{ {value_type}: {value} }}")
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct ErrorValue {
    code: usize,
    pub message: String,
}

#[derive(Debug)]
pub struct TorrentBuilder {
    path: Utf8PathBuf,
    tracker: TorrentTracker,
    piece_length: usize,
    comment: Option<String>,
    target: Option<Utf8PathBuf>,
    webseeds: Option<Vec<String>>,
    private: Option<bool>,
    created_by: Option<String>,
    trackers: Option<Vec<String>>,
}

impl TorrentBuilder {
    pub fn new(path: Utf8PathBuf, tracker: TorrentTracker, piece_length: usize) -> Self {
        Self {
            path,
            tracker,
            piece_length,
            comment: None,
            target: None,
            webseeds: None,
            private: None,
            created_by: None,
            trackers: None,
        }
    }

    pub fn with_comment(mut self, comment: String) -> Self {
        self.comment = comment.into();
        self
    }
    pub fn with_target(mut self, target: Utf8PathBuf) -> Self {
        self.target = target.into();
        self
    }
    pub fn with_webseeds(mut self, webseeds: Vec<String>) -> Self {
        self.webseeds = webseeds.into();
        self
    }
    pub fn with_author(mut self, author: String) -> Self {
        self.created_by = author.into();
        self
    }
    pub fn with_trackers(mut self, trackers: Vec<String>) -> Self {
        self.trackers = trackers.into();
        self
    }

    pub fn private(mut self, enable: bool) -> Self {
        self.private = enable.into();
        self
    }

    pub fn build(self) -> Torrent {
        Torrent {
            path: self.path,
            tracker: self.tracker,
            piece_length: self.piece_length,
            comment: self.comment,
            target: self.target,
            webseeds: self.webseeds,
            private: self.private,
            created_by: self.created_by,
            trackers: self.trackers,
        }
    }
}

#[derive(Serialize, Debug)]
pub struct Torrent {
    path: Utf8PathBuf,
    tracker: TorrentTracker,
    piece_length: usize,
    comment: Option<String>,
    target: Option<Utf8PathBuf>,
    webseeds: Option<Vec<String>>,
    private: Option<bool>,
    created_by: Option<String>,
    trackers: Option<Vec<String>>,
}
