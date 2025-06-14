use super::JsonTrack;
use crate::formats::{
    trackjson::LRAJsonArrayLine, GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2
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

    if let Some(line_list) = track.lines {
    for line in line_list {
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
  }

    // Legacy line array
    if let Some(line_list) = track.line_array {
    for line in line_list {
        match line {
            LRAJsonArrayLine::BlueLine(id, x1, y1, x2, y2, extended, flipped) => {
                let base_line = Line {
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    line_type: LineType::BLUE,
                };

                parsed_track.simulation_lines.push(SimulationLine {
                    base_line,
                    flipped,
                    left_extension: extended == 1 || extended == 3,
                    right_extension: extended == 2 || extended == 3,
                    multiplier: None,
                });
            }
            LRAJsonArrayLine::RedLine(id, x1, y1, x2, y2, extended, flipped, _, _, multiplier) => {
                let base_line = Line {
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    line_type: LineType::RED,
                };

                parsed_track.simulation_lines.push(SimulationLine {
                    base_line,
                    flipped,
                    left_extension: extended == 1 || extended == 3,
                    right_extension: extended == 2 || extended == 3,
                    multiplier: Some(multiplier as f64),
                });
            }
            LRAJsonArrayLine::GreenLine(id, x1, y1, x2, y2) => {
                let base_line = Line {
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    line_type: LineType::GREEN,
                };

                parsed_track.scenery_lines.push(SceneryLine {
                    base_line,
                    width: None,
                });
            }
        }
    }
  }

    parsed_track.start_position = Vec2 {
        x: track.start_pos.x,
        y: track.start_pos.y,
    };

    parsed_track.title = track.label;

    if let Some(creator) = track.creator {
      parsed_track.artist = creator;
    }
    if let Some(description) = track.description {
      parsed_track.description = description;
    }
    if let Some(duration) = track.duration {
      parsed_track.duration = duration;
    }
    if let Some(script) = track.script {
      parsed_track.script = script;
    }

    // TODO: These fields need parsing into the internal format still
    // start_zoom, zero_start, line_based_triggers, time_based_triggers, x_gravity, y_gravity, gravity_well_size,
    // background_color_red/green/blue, line_color_red/green/blue
    Ok(parsed_track)
}
