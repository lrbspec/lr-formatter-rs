use super::JsonTrack;
use crate::formats::{
    GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2,
};
use anyhow::{Result, anyhow};

pub fn read(json_str: &str) -> Result<InternalTrackFormat> {
    let mut parsed_track = InternalTrackFormat::filled_default();
    let track: JsonTrack = serde_json::from_str(json_str)?;

    parsed_track.grid_version = match track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => return Err(anyhow!("Invalid grid version {} when parsing json!", other)),
    };

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
            parsed_track.scenery_lines.push(SceneryLine {
                base_line,
                width: None,
            });
        } else {
            parsed_track.simulation_lines.push(SimulationLine {
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

    parsed_track.start_position = Vec2 {
        x: track.start_pos.x,
        y: track.start_pos.y,
    };

    parsed_track.title = track.label;
    parsed_track.artist = track.creator;
    parsed_track.description = track.description;
    parsed_track.duration = track.duration;
    parsed_track.script = track.script;

    Ok(parsed_track)
}
