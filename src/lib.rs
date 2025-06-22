//! This library allows parsing of various Line Rider track file formats
mod formats;
pub use formats::{internal, lrb, sol, trackjson, trk};
pub(crate) mod util;
