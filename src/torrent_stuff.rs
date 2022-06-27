use std::fmt;

use camino::Utf8PathBuf;
use reqwest::Url;
use serde::{ser::SerializeStruct, Deserialize, Serialize};

mod torrent_options;
use serde_json::{json, Value};
pub use torrent_options::*;

use crate::DelugeApiError;

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
        self.error.map_or_else(
            || self.result.ok_or(DelugeApiError::EmptyResult),
            |err| Err(err.message.into()),
        )
    }
    pub(crate) fn get_ref_result(&self) -> Result<&V, DelugeApiError> {
        self.error.as_ref().map_or_else(
            || self.result.as_ref().ok_or(DelugeApiError::EmptyResult),
            |err| Err(err.message.clone().into()),
        ) //Should look into maybe doing some stuff to make this clone unnecessary
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
        let res = self.get_ref_result();
        let (value_type, value) = match res {
            Ok(v) => ("result", format!("{v:?}")),
            Err(e) => ("error", e.to_string()),
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
    // //! Make const when const_precise_live_drops reaches stable
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
    pub const fn new(path: Utf8PathBuf, tracker: TorrentTracker, piece_length: usize) -> Self {
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

    pub fn with_comment(&mut self, comment: String) -> &mut Self {
        self.comment = comment.into();
        self
    }
    pub fn with_target(&mut self, target: Utf8PathBuf) -> &mut Self {
        self.target = target.into();
        self
    }
    pub fn with_webseeds(&mut self, webseeds: Vec<String>) -> &mut Self {
        self.webseeds = webseeds.into();
        self
    }
    pub fn with_author(&mut self, author: String) -> &mut Self {
        self.created_by = author.into();
        self
    }
    pub fn with_trackers(&mut self, trackers: Vec<String>) -> &mut Self {
        self.trackers = trackers.into();
        self
    }

    pub fn private(&mut self, enable: bool) -> &mut Self {
        self.private = enable.into();
        self
    }

    // //! Make const when const_precise_live_drops reaches stable
    #[allow(clippy::missing_const_for_fn)]
    #[must_use]
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

#[derive(Debug)]
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

impl Torrent {
    pub(crate) fn into_list(self) -> Vec<Value> {
        vec![
            json!(self.path),
            json!(self.tracker),
            json!(self.piece_length),
            json!(self.comment),
            json!(self.target),
            json!(self.webseeds),
            json!(self.private),
            json!(self.created_by),
            json!(self.trackers),
        ]
    }
}

#[derive(Deserialize, Debug)]
pub struct Account {
    pub username: String,
    pub password: String,
    pub authlevel: String,
    pub authlevel_int: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct Host {
    pub host_id: String,
    pub hostname: String,
    pub port: usize,
    pub username: String,
}

#[cfg(test)]
mod test {
    use std::error::Error;

    use crate::torrent_stuff::{temp, Host};

    #[test]
    fn test1() -> Result<(), Box<dyn Error>> {
        let json = r#"["1","2",3,"4"]"#;
        eprintln!("{}", &json);
        let json: Host = serde_json::from_str(json)?;
        eprintln!("{:?}", &json);
        assert_eq!(json.host_id, "1");
        assert_eq!(json.hostname, "2");
        assert_eq!(json.port, 3);
        assert_eq!(json.username, "4");
        Ok(())
    }

}

