//! This crate allows reading and writing Line Rider track file formats\
//! Supported formats: LRB (read/write), SOL (read/write), JSON (read/write), TRK (read)
//!
//! # Usage
//! ```rust
//! let track_bytes = fs::read_to_string("test.track.json").unwrap();
//! let track = lr_formatter_rs::trackjson::read(&track_bytes).unwrap();
//! println!("Title: {}", track.title);
//! ```

mod errors;
mod formats;
pub(crate) mod util;

pub use errors::{TrackReadError, TrackWriteError};
pub use formats::{internal, lrb, sol, trackjson, trk};
