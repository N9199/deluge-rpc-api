use std::collections::{HashMap, HashSet};

use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

type OsString = String; // ! Should just be OsString to enforce correct rules, but serialization of this is a pain

#[derive(Debug, Serialize, Deserialize)]
pub enum TorrentPriorities {
    Skip = 0,
    Low = 1,
    Normal = 4,
    High = 7,
}

#[derive(Debug)]
pub struct TorrentOptions(HashSet<TorrentOption>);

impl TorrentOptions {
    pub fn to_json(&self) -> Value {
        Value::Object(self.0.iter().map(|value| value.to_json()).collect())
    }
    pub fn new() -> Self {
        Self(HashSet::new())
    }
    pub fn insert(&mut self, value: TorrentOption) {
        self.0.insert(value);
    }
    pub fn replace(&mut self, value: TorrentOption) -> Option<TorrentOption> {
        self.0.replace(value)
    }
    pub fn remove(&mut self, value: &TorrentOption) -> bool {
        self.0.remove(value)
    }
    pub fn take(&mut self, value: &TorrentOption) -> Option<TorrentOption> {
        self.0.take(value)
    }
}

impl Default for TorrentOptions {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Derivative, Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
#[derivative(Hash, PartialEq, Eq)]
pub enum TorrentOption {
    AddPaused(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    AutoManaged(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    DownloadLocation(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        OsString, 
    ),
    FilePriorities(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        Vec<TorrentPriorities>,
    ),
    MappedFiles(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        HashMap<i32, OsString>, 
    ),
    MaxConnections(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        i64,
    ),
    MaxDownloadSpeed(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        f64,
    ),
    MaxUploadSlots(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        i64,
    ),
    MaxUploadSpeed(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        f64,
    ),
    MoveCompleted(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    MoveCompletedPath(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        OsString, 
    ),
    Name(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        String,
    ),
    Owner(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        String,
    ),
    PreAllocateStorage(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    PrioritizeFirstLastPieces(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    RemoveAtRatio(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    SeedMode(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    SequentialDownload(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    Shared(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    StopAtRatio(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
    StopRatio(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        f64,
    ),
    SuperSeeding(
        #[derivative(PartialEq = "ignore")]
        #[derivative(Hash = "ignore")]
        bool,
    ),
}

impl TorrentOption {
    fn to_json(&self) -> (String, Value) {
        (self.to_string(), self.get_inner())
    }
    // Look into more generic, shorter way to handle this
    fn get_inner(&self) -> Value {
        match self {
            TorrentOption::AddPaused(value) => json!(value),
            TorrentOption::AutoManaged(value) => json!(value),
            TorrentOption::DownloadLocation(value) => json!(value),
            TorrentOption::FilePriorities(value) => json!(value),
            TorrentOption::MappedFiles(value) => json!(value),
            TorrentOption::MaxConnections(value) => json!(value),
            TorrentOption::MaxDownloadSpeed(value) => json!(value),
            TorrentOption::MaxUploadSlots(value) => json!(value),
            TorrentOption::MaxUploadSpeed(value) => json!(value),
            TorrentOption::MoveCompleted(value) => json!(value),
            TorrentOption::MoveCompletedPath(value) => json!(value),
            TorrentOption::Name(value) => json!(value),
            TorrentOption::Owner(value) => json!(value),
            TorrentOption::PreAllocateStorage(value) => json!(value),
            TorrentOption::PrioritizeFirstLastPieces(value) => json!(value),
            TorrentOption::RemoveAtRatio(value) => json!(value),
            TorrentOption::SeedMode(value) => json!(value),
            TorrentOption::SequentialDownload(value) => json!(value),
            TorrentOption::Shared(value) => json!(value),
            TorrentOption::StopAtRatio(value) => json!(value),
            TorrentOption::StopRatio(value) => json!(value),
            TorrentOption::SuperSeeding(value) => json!(value),
        }
    }
}
