use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Read};

use crate::{
    TrackReadError,
    formats::sol::amf0::deserialize,
    track::{GridVersion, Line, LineType, SceneryLine, SimulationLine, Track},
    util::{StringLength, bytes_to_hex_string, parse_string},
};

pub fn read(data: &[u8], track_index: Option<u32>) -> Result<Track, TrackReadError> {
    let mut internal = Track::new();
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 2];
    cursor.read_exact(&mut magic_number)?;

    if magic_number != [0x00, 0xBF] {
        return Err(TrackReadError::InvalidData {
            name: "magic number".to_string(),
            value: bytes_to_hex_string(&magic_number),
        });
    }

    // Header
    let _file_size = cursor.read_u32::<BigEndian>()? + 6;

    let mut tag = [0u8; 4];
    cursor.read_exact(&mut tag)?;

    if tag != [b'T', b'C', b'S', b'O'] {
        return Err(TrackReadError::InvalidData {
            name: "header tag".to_string(),
            value: bytes_to_hex_string(&tag),
        });
    }

    let mut marker = [0u8; 6];
    cursor.read_exact(&mut marker)?;
    if marker != [0x00, 0x04, 0x00, 0x00, 0x00, 0x00] {
        return Err(TrackReadError::InvalidData {
            name: "header marker".to_string(),
            value: bytes_to_hex_string(&marker),
        });
    }

    let sol_name = parse_string::<BigEndian>(&mut cursor, StringLength::U16)?;
    if sol_name.as_str() != "savedLines" {
        return Err(TrackReadError::InvalidData {
            name: "sol name".to_string(),
            value: sol_name.to_string(),
        });
    }

    let _padding = cursor.read_u32::<BigEndian>()?;

    let data_name = parse_string::<BigEndian>(&mut cursor, StringLength::U16)?;
    if data_name.as_str() != "trackList" {
        return Err(TrackReadError::InvalidData {
            name: "data name".to_string(),
            value: data_name.to_string(),
        });
    }

    // Track Data
    let current_pos = cursor.position() as usize;
    let trimmed = &data[current_pos..data.len().saturating_sub(1)]; // trim off the last byte \x00
    cursor = Cursor::new(trimmed);

    let result = &deserialize(&mut cursor)?;
    let track_list_amf = &result[0];
    let track_list =
        track_list_amf
            .clone()
            .get_object_properties()
            .ok_or(TrackReadError::InvalidData {
                name: "track list".to_string(),
                value: format!("{:?}", track_list_amf),
            })?;

    let target_track_index = match track_index {
        Some(index) => &index.to_string(),
        None => "0",
    };

    let target_track_amf =
        track_list
            .get(target_track_index)
            .ok_or(TrackReadError::InvalidData {
                name: "track index".to_string(),
                value: format!("{:?}", target_track_index),
            })?;

    let target_track =
        target_track_amf
            .clone()
            .get_object_properties()
            .ok_or(TrackReadError::InvalidData {
                name: "track".to_string(),
                value: format!("{:?}", target_track_amf),
            })?;

    if let Some(val) = target_track.get("label") {
        internal.title = val
            .clone()
            .get_string()
            .ok_or(TrackReadError::InvalidData {
                name: "label".to_string(),
                value: format!("{:?}", val),
            })?;
    }

    if let Some(val) = target_track.get("version") {
        let version_string = val
            .clone()
            .get_string()
            .ok_or(TrackReadError::InvalidData {
                name: "grid version".to_string(),
                value: format!("{:?}", val),
            })?;

        internal.grid_version = match version_string.as_str() {
            "6.0" => GridVersion::V6_0,
            "6.1" => GridVersion::V6_1,
            "6.2" => GridVersion::V6_2,
            other => {
                return Err(TrackReadError::InvalidData {
                    name: "grid version".to_string(),
                    value: other.to_string(),
                });
            }
        }
    } else {
        internal.grid_version = GridVersion::V6_0
    }

    if let Some(val) = target_track.get("startLine") {
        let start_position =
            val.clone()
                .get_object_properties()
                .ok_or(TrackReadError::InvalidData {
                    name: "start line".to_string(),
                    value: format!("{:?}", val),
                })?;

        let start_x_amf = start_position.get("0").ok_or(TrackReadError::InvalidData {
            name: "start line x".to_string(),
            value: format!("{:?}", start_position),
        })?;
        internal.start_position.x =
            start_x_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "start x value".to_string(),
                    value: format!("{:?}", start_x_amf),
                })?;

        let start_y_amf = start_position.get("1").ok_or(TrackReadError::InvalidData {
            name: "start line y".to_string(),
            value: format!("{:?}", start_position),
        })?;
        internal.start_position.y =
            start_y_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "start y value".to_string(),
                    value: format!("{:?}", start_y_amf),
                })?;
    }

    if target_track.contains_key("trackData") {
        let rider = internal.riders.get_mut(0).ok_or(TrackReadError::Other {
            message: "Internal track should have contained an initial rider".to_string(),
        })?;
        rider.start_velocity.x = 0.0;
        rider.start_velocity.y = 0.0;
    }

    if let Some(val) = target_track.get("data") {
        let lines_list =
            val.clone()
                .get_object_properties()
                .ok_or(TrackReadError::InvalidData {
                    name: "lines list".to_string(),
                    value: format!("{:?}", val),
                })?;

        for line_amf in lines_list.values() {
            let line =
                line_amf
                    .clone()
                    .get_object_properties()
                    .ok_or(TrackReadError::InvalidData {
                        name: "line".to_string(),
                        value: format!("{:?}", line_amf),
                    })?;

            let x1_amf = line.get("0").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let x1 = x1_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "line x1".to_string(),
                    value: format!("{:?}", x1_amf),
                })?;

            let y1_amf = line.get("1").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let y1 = y1_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "line y1".to_string(),
                    value: format!("{:?}", y1_amf),
                })?;

            let x2_amf = line.get("2").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let x2 = x2_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "line x2".to_string(),
                    value: format!("{:?}", x2_amf),
                })?;

            let y2_amf = line.get("3").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let y2 = y2_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "line y2".to_string(),
                    value: format!("{:?}", y2_amf),
                })?;

            let ext_amf = line.get("4").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let ext = ext_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "line extension".to_string(),
                    value: format!("{:?}", ext_amf),
                })?;

            let left_extension = ext == 1.0 || ext == 3.0;
            let right_extension = ext == 2.0 || ext == 3.0;

            let flipped_amf = line.get("5").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let flipped = flipped_amf
                .clone()
                .get_boolean()
                .or_else(|| flipped_amf.clone().get_number().map(|num| num == 1.0))
                .ok_or(TrackReadError::InvalidData {
                    name: "line flipped".to_string(),
                    value: format!("{:?}", flipped_amf),
                })?;

            let id_amf = line.get("8").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let id_float = id_amf
                .clone()
                .get_number()
                .ok_or(TrackReadError::InvalidData {
                    name: "line id".to_string(),
                    value: format!("{:?}", id_amf),
                })?;

            let unsafe_id =
                if id_float.is_finite() && id_float >= 0.0 && id_float <= u32::MAX as f64 {
                    Some(id_float as u32)
                } else {
                    None
                };

            let id = match unsafe_id {
                Some(val) => val,
                None => {
                    return Err(TrackReadError::InvalidData {
                        name: "line id".to_string(),
                        value: id_float.to_string(),
                    });
                }
            };

            let line_type_amf = line.get("9").ok_or(TrackReadError::InvalidData {
                name: "line".to_string(),
                value: format!("{:?}", line),
            })?;

            let line_type_numeric =
                line_type_amf
                    .clone()
                    .get_number()
                    .ok_or(TrackReadError::InvalidData {
                        name: "line type".to_string(),
                        value: format!("{:?}", line_type_amf),
                    })?;

            let line_type = match line_type_numeric {
                0.0 => LineType::Standard,
                1.0 => LineType::Acceleration,
                2.0 => LineType::Scenery,
                other => {
                    return Err(TrackReadError::InvalidData {
                        name: "line type".to_string(),
                        value: other.to_string(),
                    });
                }
            };

            let base_line = Line {
                x1,
                y1,
                x2,
                y2,
                id,
                line_type,
            };

            if line_type == LineType::Scenery {
                internal.scenery_lines.push(SceneryLine {
                    base_line,
                    width: None,
                });
            } else {
                internal.simulation_lines.push(SimulationLine {
                    base_line,
                    flipped,
                    left_extension,
                    right_extension,
                    multiplier: None,
                });
            }
        }
    }

    Ok(internal)
}
