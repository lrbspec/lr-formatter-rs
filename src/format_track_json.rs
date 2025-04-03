use crate::format_internal::{
    GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2,
};
use anyhow::{Result, anyhow};
use serde::{Deserialize, Serialize};

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

pub fn parse_track_json(json_str: &str) -> Result<InternalTrackFormat> {
    let track: JsonTrack = serde_json::from_str(json_str)?;

    let grid_version = match track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => return Err(anyhow!("Invalid grid version {} when parsing json!", other)),
    };

    let mut scenery_lines = Vec::<SceneryLine>::new();
    let mut simulation_lines = Vec::<SimulationLine>::new();

    for line in track.lines {
        let line_type = match line.line_type {
            0 => LineType::BLUE,
            1 => LineType::RED,
            2 => LineType::GREEN,
            other => return Err(anyhow!("Json line had invalid line type {}!", other)),
        };

        let base_line = Line {
            id: line.id,
            x1: line.x1,
            y1: line.y1,
            x2: line.x2,
            y2: line.y2,
            line_type,
        };

        if line.line_type == 2 {
            scenery_lines.push(SceneryLine {
                base_line,
                width: None,
            });
        } else {
            simulation_lines.push(SimulationLine {
                base_line,
                flipped: line
                    .flipped
                    .ok_or_else(|| anyhow!("Json simline did not have flipped attribute!"))?,
                left_extension: line.left_ext.ok_or_else(|| {
                    anyhow!("Json simline did not have left_extension attribute!")
                })?,
                right_extension: line.right_ext.ok_or_else(|| {
                    anyhow!("Json simline did not have right_extension attribute!")
                })?,
                multiplier: None,
            });
        }
    }

    Ok(InternalTrackFormat {
        title: track.label,
        grid_version,
        start_position: track.start_pos,
        scenery_lines,
        simulation_lines,
        artist: track.creator,
        description: track.description,
        duration: track.duration,
        script: track.script,
    })
}

pub fn write_track_json(internal: &InternalTrackFormat) -> Result<String> {
    let version = match internal.grid_version {
        GridVersion::V6_0 => String::from("6.0"),
        GridVersion::V6_1 => String::from("6.1"),
        GridVersion::V6_2 => String::from("6.2"),
    };

    let mut lines = Vec::<JsonLine>::new();

    for line in &internal.simulation_lines {
        let line_type = if line.base_line.line_type == LineType::BLUE {
            0u8
        } else {
            1u8
        };

        lines.push(JsonLine {
            id: line.base_line.id,
            line_type,
            x1: line.base_line.x1,
            y1: line.base_line.y1,
            x2: line.base_line.x2,
            y2: line.base_line.y2,
            flipped: Some(line.flipped),
            left_ext: Some(line.left_extension),
            right_ext: Some(line.right_extension),
            multiplier: line.multiplier,
            width: None,
        });
    }

    for line in &internal.scenery_lines {
        lines.push(JsonLine {
            id: line.base_line.id,
            line_type: 2,
            x1: line.base_line.x1,
            y1: line.base_line.y1,
            x2: line.base_line.x2,
            y2: line.base_line.y2,
            flipped: None,
            left_ext: None,
            right_ext: None,
            multiplier: None,
            width: line.width,
        });
    }

    let track = JsonTrack {
        label: internal.title.clone(),
        version,
        start_pos: internal.start_position.clone(),
        lines,
        creator: internal.artist.clone(),
        description: internal.description.clone(),
        duration: internal.duration,
        script: internal.script.clone(),
        layers: Vec::<JsonLayer>::new(),
        riders: Vec::<JsonRider>::new(),
    };

    Ok(serde_json::to_string_pretty(&track)?)
}
