pub mod reader;

pub use reader::read;

use once_cell::sync::Lazy;
use std::collections::HashSet;

const FEATURE_RED_MULTIPLIER: &str = "REDMULTIPLIER";
const FEATURE_SCENERY_WIDTH: &str = "SCENERYWIDTH";
const FEATURE_SONG_INFO: &str = "SONGINFO";
const FEATURE_IGNORABLE_TRIGGER: &str = "IGNORABLE_TRIGGER";
const FEATURE_6_1: &str = "6.1";
const FEATURE_ZERO_START: &str = "ZEROSTART";
const FEATURE_REMOUNT: &str = "REMOUNT";
const FEATURE_FRICTIONLESS: &str = "FRICTIONLESS";

pub static KNOWN_FEATURES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        FEATURE_RED_MULTIPLIER,
        FEATURE_SCENERY_WIDTH,
        FEATURE_SONG_INFO,
        FEATURE_IGNORABLE_TRIGGER,
        FEATURE_6_1,
        FEATURE_ZERO_START,
        FEATURE_REMOUNT,
        FEATURE_FRICTIONLESS,
    ])
});
