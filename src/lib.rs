//! This library allows parsing of various Line Rider track file formats
//! Supported reading formats: LRB, SOL, JSON, TRK
//! Supported writing formats: LRB, SOL, JSON
//!
//! # Usage
//! ```rust
//! let track_bytes = fs::read_to_string("./fixtures/crash.track.json").unwrap();
//! let track = lr_formatter_rs::trackjson::read(&track_bytes).unwrap();
//! println!("Title: {}", track.title);
//! ```

mod formats;
pub use formats::{internal, lrb, sol, trackjson, trk};
pub(crate) mod util;
