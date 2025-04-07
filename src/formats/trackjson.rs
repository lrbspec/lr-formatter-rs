use crate::formats::internal::Vec2;
use serde::{Deserialize, Serialize};

pub mod reader;
pub mod writer;

// A u32 value that can take the range of a normal u32, or -1 for invalid (for parsing some json fields)
#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
enum FaultyU32 {
    Valid(u32),
    Invalid(i32),
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonLine {
    id: u32,
    #[serde(rename = "type")]
    line_type: u8,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    flipped: Option<bool>,
    #[serde(rename = "leftExtended", skip_serializing_if = "Option::is_none")]
    left_ext: Option<bool>,
    #[serde(rename = "rightExtended", skip_serializing_if = "Option::is_none")]
    right_ext: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    multiplier: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonLayer {
    id: u32,
    #[serde(rename = "type")]
    layer_type: u8,
    name: String,
    visible: bool,
    editable: bool,
    #[serde(rename = "folderId", skip_serializing_if = "Option::is_none")]
    folder_id: Option<FaultyU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonRider {
    #[serde(rename = "startPosition")]
    start_pos: Vec2,
    #[serde(rename = "startVelocity")]
    start_vel: Vec2,
    #[serde(skip_serializing_if = "Option::is_none")]
    angle: Option<f64>,
    remountable: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct JsonTrack {
    label: String,
    creator: String,
    description: String,
    duration: u32,
    version: String,
    #[serde(rename = "startPosition")]
    start_pos: Vec2,
    lines: Vec<JsonLine>,
    layers: Vec<JsonLayer>,
    riders: Vec<JsonRider>,
    script: String,
}
