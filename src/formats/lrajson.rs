pub mod deserialize_line;
pub mod reader;

pub use reader::read;

use serde::Deserialize;

// A u32 value that can take the range of a normal u32, or negative for invalid (for parsing some json fields)
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum FaultyU32 {
    Valid(u32),
    Invalid(i32),
}

#[derive(Debug, Deserialize)]
struct Vec2 {
    x: f64,
    y: f64,
}

// LRA line array types:
// [type: 0, id: int, x1: double, y1: double, x2: double, y2: double, extended: u8, flipped: bool]
// [type: 1, id: int, x1: double, y1: double, x2: double, y2: double, extended: u8, flipped: bool, _?: -1, _?: -1, multiplier?: int]
// [type: 2, id: int, x1: double, y1: double, x2: double, y2: double]
// Extended bitflags 0b000000ba
// a: 1 if starting/left extension
// b: 1 if ending/right extension
#[derive(Debug)]
enum LRAJsonArrayLine {
    BlueLine(u32, f64, f64, f64, f64, u8, bool),
    RedLine(u32, f64, f64, f64, f64, u8, bool, (), (), u32),
    GreenLine(u32, f64, f64, f64, f64),
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct LRAJsonLegacyZoomTrigger {
    #[serde(rename = "ID")]
    id: u32,
    zoom: bool,  // whether zoom trigger enabled
    target: f32, // target to zoom to
    frames: u32, // duration of zoom
}

// Faulty U32's are used here whenever properties are -999, which
// represents undefined/unused in the LRA json trigger format
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct LRAJsonTrigger {
    #[serde(rename = "triggerType")]
    trigger_type: u8,
    start: u32,
    end: u32,
    #[serde(rename = "zoomTarget")]
    zoom_target: FaultyU32,
    #[serde(rename = "backgroundRed")]
    background_red: Option<FaultyU32>,
    #[serde(rename = "backgroundGreen")]
    background_green: Option<FaultyU32>,
    #[serde(rename = "backgroundBlue")]
    background_blue: Option<FaultyU32>,
    #[serde(rename = "lineRed")]
    line_red: Option<FaultyU32>,
    #[serde(rename = "lineGreen")]
    line_green: Option<FaultyU32>,
    #[serde(rename = "lineBlue")]
    line_blue: Option<FaultyU32>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct LRAJsonTrack {
    label: String,
    version: String,
    #[serde(rename = "startPosition")]
    start_pos: Vec2,
    #[serde(rename = "linesArray")]
    line_array: Vec<LRAJsonArrayLine>,
    #[serde(rename = "startZoom")]
    start_zoom: Option<f32>,
    #[serde(rename = "zeroStart")]
    zero_start: Option<bool>,
    #[serde(rename = "triggers")]
    line_based_triggers: Option<Vec<LRAJsonLegacyZoomTrigger>>,
    #[serde(rename = "gameTriggers")]
    time_based_triggers: Option<Vec<LRAJsonTrigger>>,
    #[serde(rename = "xGravity")]
    x_gravity: Option<f32>,
    #[serde(rename = "yGravity")]
    y_gravity: Option<f32>,
    #[serde(rename = "gravityWellSize")]
    gravity_well_size: Option<f64>,
    #[serde(rename = "bgR")]
    background_color_red: Option<u32>,
    #[serde(rename = "bgG")]
    background_color_green: Option<u32>,
    #[serde(rename = "bgB")]
    background_color_blue: Option<u32>,
    #[serde(rename = "lineR")]
    line_color_red: Option<u32>,
    #[serde(rename = "lineG")]
    line_color_green: Option<u32>,
    #[serde(rename = "lineB")]
    line_color_blue: Option<u32>,
}
