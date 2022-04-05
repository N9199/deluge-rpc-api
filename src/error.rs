use thiserror::Error;


#[derive(Error, Debug)]
pub enum DelugeError{
    #[error("{0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("TorrentResponse JSON doesn't satisfy response schema")]
    Json,
    #[error("{0}")]
    TryInto(#[from] std::num::TryFromIntError)
}