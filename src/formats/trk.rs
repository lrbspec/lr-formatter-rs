//! Format used by [Line Rider: Advanced](https://github.com/jealouscloud/linerider-advanced) and its forks

mod reader;

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
const FEATURE_START_ZOOM: &str = "STARTZOOM";
const FEATURE_X_GRAVITY: &str = "XGRAVITY";
const FEATURE_Y_GRAVITY: &str = "YGRAVITY";
const FEATURE_GRAVITY_WELL_SIZE: &str = "GRAVITYWELLSIZE";
const FEATURE_BACKGROUND_COLOR_R: &str = "BGCOLORR";
const FEATURE_BACKGROUND_COLOR_G: &str = "BGCOLORG";
const FEATURE_BACKGROUND_COLOR_B: &str = "BGCOLORB";
const FEATURE_LINE_COLOR_R: &str = "LINECOLORR";
const FEATURE_LINE_COLOR_G: &str = "LINECOLORG";
const FEATURE_LINE_COLOR_B: &str = "LINECOLORB";
const FEATURE_TRIGGERS: &str = "TRIGGERS";

static KNOWN_FEATURES: Lazy<HashSet<&'static str>> = Lazy::new(|| {
    HashSet::from([
        FEATURE_RED_MULTIPLIER,
        FEATURE_SCENERY_WIDTH,
        FEATURE_SONG_INFO,
        FEATURE_IGNORABLE_TRIGGER,
        FEATURE_6_1,
        FEATURE_ZERO_START,
        FEATURE_REMOUNT,
        FEATURE_FRICTIONLESS,
        FEATURE_START_ZOOM,
        FEATURE_X_GRAVITY,
        FEATURE_Y_GRAVITY,
        FEATURE_GRAVITY_WELL_SIZE,
        FEATURE_BACKGROUND_COLOR_R,
        FEATURE_BACKGROUND_COLOR_G,
        FEATURE_BACKGROUND_COLOR_B,
        FEATURE_LINE_COLOR_R,
        FEATURE_LINE_COLOR_G,
        FEATURE_LINE_COLOR_B,
        FEATURE_TRIGGERS,
    ])
});
