//! This crate provides safe reading and writing of Line Rider track file formats\
//! Supported formats: LRB (read/write), SOL (read/write), JSON (read/write), TRK (read)
//!
//! # Usage
//! ```rust
//! // Note: `.unwrap()` is used here for example purposes only
//! use std::{fs::{self, File}, io::Write};
//!
//! // Read a track
//! let track_json_bytes = fs::read_to_string("../examples/samples/Omniverse2.track.json").unwrap();
//! let mut track = lr_formatter_rs::trackjson::read(&track_json_bytes).unwrap();
//!
//! // Do things to the track
//! println!("Loaded track: {}", track.title);
//! track.duration = 40;
//!
//! // Write the track (in a different format, if you prefer)
//! let track_lrb_bytes = lr_formatter_rs::lrb::write(&track).unwrap();
//! // (Commented out for Doc-tests)
//! // File::create("test_track.lrb").unwrap().write_all(&track_lrb_bytes).unwrap()
//! ```

mod formats;
mod track;
pub(crate) mod util;
pub use formats::{TrackReadError, TrackWriteError, lrb, sol, trackjson, trk};
pub use track::{Track, TrackBuilder, TrackBuilderError, TrackFeature};
