use super::{JsonLayer, JsonLine, JsonRider, JsonTrack, Vec2};
use crate::formats::{GridVersion, InternalTrackFormat, LineType};
use anyhow::Result;

pub fn write(internal: &InternalTrackFormat) -> Result<String> {
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

    let start_pos = Vec2 {
        x: internal.start_position.x,
        y: internal.start_position.y,
    };

    let track = JsonTrack {
        label: internal.title.clone(),
        version,
        start_pos,
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
