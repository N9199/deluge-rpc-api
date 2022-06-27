use std::collections::HashMap;

use camino::Utf8PathBuf;
use derivative::Derivative;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use crate::enum_map::{EnumMap, SerializableEnum};

// Every value **must** fit in 7bits, as it's cast as an i8.
#[derive(Clone, Copy, Debug, Deserialize)]
#[repr(u8)]
pub enum TorrentPriorities {
    Skip = 0,
    Low = 1,
    Normal = 4,
    High = 7,
}

impl Serialize for TorrentPriorities {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        #[allow(clippy::cast_possible_wrap)]
        serializer.serialize_i8((*self as u8) as i8)
    }
}

pub type TorrentOptions = EnumMap<TorrentOption>;

#[derive(Derivative, Debug, strum::Display)]
#[strum(serialize_all = "snake_case")]
pub enum TorrentOption {
    AddPaused(bool),
    AutoManaged(bool),
    DownloadLocation(Utf8PathBuf),
    FilePriorities(Vec<TorrentPriorities>),
    MappedFiles(HashMap<i32, Utf8PathBuf>),
    MaxConnections(i64),
    MaxDownloadSpeed(f64),
    MaxUploadSlots(i64),
    MaxUploadSpeed(f64),
    MoveCompleted(bool),
    MoveCompletedPath(Utf8PathBuf),
    Name(String),
    Owner(String),
    PreAllocateStorage(bool),
    PrioritizeFirstLastPieces(bool),
    RemoveAtRatio(bool),
    SeedMode(bool),
    SequentialDownload(bool),
    Shared(bool),
    StopAtRatio(bool),
    StopRatio(f64),
    SuperSeeding(bool),
}

impl TorrentOption {
    // Look into more generic, shorter way to handle this
    fn get_inner(&self) -> Value {
        match self {
            TorrentOption::DownloadLocation(value) | TorrentOption::MoveCompletedPath(value) => {
                json!(value)
            }
            TorrentOption::Name(value) | TorrentOption::Owner(value) => json!(value),
            TorrentOption::FilePriorities(value) => json!(value),
            TorrentOption::MappedFiles(value) => json!(value),
            TorrentOption::MaxConnections(value) | TorrentOption::MaxUploadSlots(value) => {
                json!(value)
            }
            TorrentOption::AddPaused(value)
            | TorrentOption::AutoManaged(value)
            | TorrentOption::MoveCompleted(value)
            | TorrentOption::PreAllocateStorage(value)
            | TorrentOption::PrioritizeFirstLastPieces(value)
            | TorrentOption::RemoveAtRatio(value)
            | TorrentOption::SeedMode(value)
            | TorrentOption::SequentialDownload(value)
            | TorrentOption::Shared(value)
            | TorrentOption::StopAtRatio(value)
            | TorrentOption::SuperSeeding(value) => json!(value),
            TorrentOption::MaxDownloadSpeed(value)
            | TorrentOption::MaxUploadSpeed(value)
            | TorrentOption::StopRatio(value) => json!(value),
        }
    }
    fn get_name(&self) -> String {
        self.to_string()
    }
}

impl SerializableEnum for TorrentOption {
    type K = String;

    type V = Value;

    fn get_key(&self) -> Self::K {
        self.get_name()
    }

    fn get_value(&self) -> Self::V {
        self.get_inner()
    }
}

#[cfg(test)]
mod test {
    use camino::Utf8PathBuf;
    use serde_json::json;

    use super::{TorrentOption, TorrentOptions};

    #[test]
    fn test() {
        let mut options = TorrentOptions::new();
        options.insert(TorrentOption::MaxConnections(32));
        options.insert(TorrentOption::MaxConnections(31));
        options.insert(TorrentOption::MoveCompletedPath(Utf8PathBuf::from("path")));

        assert_eq!(
            r#"{"max_connections":31,"move_completed_path":"path"}"#,
            format!("{}", json!(options))
        );
    }
}
