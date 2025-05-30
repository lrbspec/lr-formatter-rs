pub mod reader;

use std::collections::HashSet;

use anyhow::{Result, bail};
use once_cell::sync::Lazy;
pub use reader::read;

use super::LineType;

const FEATURE_RED_MULTIPLIER: &str = "REDMULTIPLIER";
const FEATURE_SCENERY_WIDTH: &str = "SCENERYWIDTH";
const FEATURE_SONG_INFO: &str = "SONGINFO";
const FEATURE_IGNORABLE_TRIGGER: &str = "IGNORABLE_TRIGGER";
const FEATURE_6_1: &str = "6.1";

pub static KNOWN_FEATURES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        FEATURE_RED_MULTIPLIER,
        FEATURE_SCENERY_WIDTH,
        FEATURE_SONG_INFO,
        FEATURE_IGNORABLE_TRIGGER,
        FEATURE_6_1,
    ])
});

fn classic_line_type(line_type: u8) -> Result<LineType> {
    match line_type {
        1 => Ok(LineType::BLUE),
        2 => Ok(LineType::RED),
        0 => Ok(LineType::GREEN),
        _ => bail!("Unknown line type!"),
    }
}
