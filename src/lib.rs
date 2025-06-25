//! This crate provides safe reading and writing of Line Rider track file formats\
//! Supported formats: LRB (read/write), SOL (read/write), JSON (read/write), TRK (read)
//!
//! # Usage
//! ```rust
//! // Note: `.unwrap()` is used here for example purposes only
//! let track_json_bytes = fs::read_to_string("test.track.json").unwrap();
//! let track = lr_formatter_rs::trackjson::read(&track_json_bytes).unwrap();
//! println!("Title: {}", track.title);
//! let track_lrb_bytes = lr_formatter_rs::lrb::write(&track).unwrap();
//! fs::File::create("test_track.lrb").unwrap().write_all(track_lrb_bytes).unwrap()
//! ```

mod errors;
mod formats;
pub(crate) mod util;

pub use errors::{TrackReadError, TrackWriteError};
pub use formats::{internal, lrb, sol, trackjson, trk};
