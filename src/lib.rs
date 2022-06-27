#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::cargo)]
#![warn(clippy::nursery)]
// #![warn(missing_docs)] //! Uncomment this after everything is at least functional.

#![allow(clippy::module_name_repetitions)]
#![allow(clippy::missing_errors_doc)]
#![allow(clippy::missing_panics_doc)] //! Remove when everything is functional
pub mod deluge_interface;
pub mod torrent_stuff;
pub use error::{DelugeApiError, DelugeError};
mod error;
mod enum_map;
mod utils;
