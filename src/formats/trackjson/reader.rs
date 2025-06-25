use super::JsonTrack;
use crate::{
    TrackReadError,
    formats::{
        internal::{
            GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2,
        },
        trackjson::LRAJsonArrayLine,
    },
};

pub fn read(json_str: &str) -> Result<InternalTrackFormat, TrackReadError> {
    let mut internal = InternalTrackFormat::new();
    let json_track: JsonTrack =
        serde_json::from_str(json_str).map_err(|err| TrackReadError::Other {
            message: format!("Failed to deserialize json track: {}", err),
        })?;

    internal.grid_version = match json_track.version.as_str() {
        "6.0" => GridVersion::V6_0,
        "6.1" => GridVersion::V6_1,
        "6.2" => GridVersion::V6_2,
        other => {
            return Err(TrackReadError::InvalidData {
                name: "grid version".to_string(),
                value: other.to_string(),
            });
        }
    };

    if let Some(line_list) = json_track.lines {
        for line in line_list {
            let line_type = match line.line_type {
                0 => LineType::BLUE,
                1 => LineType::RED,
                2 => LineType::GREEN,
                other => {
                    return Err(TrackReadError::InvalidData {
                        name: "line type".to_string(),
                        value: other.to_string(),
                    });
                }
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
                internal.scenery_lines.push(SceneryLine {
                    base_line,
                    width: line.width,
                });
            } else {
                let (left_extension, right_extension) = if let Some(ext) = line.extended {
                    (ext & 1 != 0, ext & 2 != 0)
                } else if let (Some(left_ext), Some(right_ext)) = (line.left_ext, line.right_ext) {
                    (left_ext, right_ext)
                } else {
                    return Err(TrackReadError::InvalidData {
                        name: "line extension".to_string(),
                        value: "None".to_string(),
                    });
                };

                internal.simulation_lines.push(SimulationLine {
                    base_line,
                    flipped: line.flipped.ok_or_else(|| TrackReadError::InvalidData {
                        name: "line flipped".to_string(),
                        value: format!("{:?}", line.flipped),
                    })?,
                    left_extension,
                    right_extension,
                    multiplier: line.multiplier,
                });
            }
        }
    }

    // Legacy line array
    if let Some(line_list) = json_track.line_array {
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

                    internal.simulation_lines.push(SimulationLine {
                        base_line,
                        flipped,
                        left_extension: extended == 1 || extended == 3,
                        right_extension: extended == 2 || extended == 3,
                        multiplier: None,
                    });
                }
                LRAJsonArrayLine::RedLine(
                    id,
                    x1,
                    y1,
                    x2,
                    y2,
                    extended,
                    flipped,
                    _,
                    _,
                    multiplier,
                ) => {
                    let base_line = Line {
                        id,
                        x1,
                        y1,
                        x2,
                        y2,
                        line_type: LineType::RED,
                    };

                    internal.simulation_lines.push(SimulationLine {
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

                    internal.scenery_lines.push(SceneryLine {
                        base_line,
                        width: None,
                    });
                }
            }
        }
    }

    internal.start_position = Vec2 {
        x: json_track.start_pos.x,
        y: json_track.start_pos.y,
    };

    internal.title = json_track.label;

    if let Some(creator) = json_track.creator {
        internal.artist = creator;
    }

    if let Some(description) = json_track.description {
        internal.description = description;
    }

    if let Some(duration) = json_track.duration {
        internal.duration = duration;
    }

    if let Some(script) = json_track.script {
        internal.script = script;
    }

    // TODO: These fields need parsing into the internal format still
    // start_zoom, zero_start, line_based_triggers, time_based_triggers, x_gravity, y_gravity, gravity_well_size,
    // background_color_red/green/blue, line_color_red/green/blue
    Ok(internal)
}
