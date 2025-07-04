use std::{
    collections::HashSet,
    io::{Cursor, Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    TrackReadError,
    formats::{
        internal::{GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine},
        trk::{
            FEATURE_BACKGROUND_COLOR_B, FEATURE_BACKGROUND_COLOR_G, FEATURE_BACKGROUND_COLOR_R,
            FEATURE_GRAVITY_WELL_SIZE, FEATURE_LINE_COLOR_B, FEATURE_LINE_COLOR_G,
            FEATURE_START_ZOOM, FEATURE_TRIGGERS, FEATURE_X_GRAVITY, FEATURE_Y_GRAVITY,
            KNOWN_FEATURES,
        },
    },
    util::{StringLength, bytes_to_hex_string, parse_string},
};

use super::{
    FEATURE_6_1, FEATURE_IGNORABLE_TRIGGER, FEATURE_LINE_COLOR_R, FEATURE_RED_MULTIPLIER,
    FEATURE_SCENERY_WIDTH, FEATURE_SONG_INFO,
};

pub fn read(data: &[u8]) -> Result<InternalTrackFormat, TrackReadError> {
    let mut parsed_track = InternalTrackFormat::new();
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if &magic_number != &[b'T', b'R', b'K', 0xF2] {
        return Err(TrackReadError::InvalidData {
            name: "magic number".to_string(),
            value: bytes_to_hex_string(&magic_number),
        });
    }

    // Version
    let version = cursor.read_u8()?;

    if version > 1 {
        return Err(TrackReadError::InvalidData {
            name: "version".to_string(),
            value: version.to_string(),
        });
    }

    let feature_string = parse_string::<LittleEndian>(&mut cursor, StringLength::U16)?;
    let mut included_features: HashSet<&str> = Default::default();

    for feature in feature_string.split(';').filter(|s| !s.is_empty()) {
        if KNOWN_FEATURES.contains(feature) {
            included_features.insert(feature);
        } else {
            return Err(TrackReadError::InvalidData {
                name: "feature".to_string(),
                value: feature.to_string(),
            });
        }
    }

    parsed_track.grid_version = if included_features.contains(FEATURE_6_1) {
        GridVersion::V6_1
    } else {
        GridVersion::V6_2
    };

    if included_features.contains(FEATURE_SONG_INFO) {
        let mut song_string_length = 0;
        let mut bit_shift = 0;

        loop {
            // Read 7BitEncodedInt song string length
            let byte = cursor.read_u8()?;
            song_string_length |= ((byte & 0x7F) as usize) << bit_shift;

            if byte & 0x80 == 0 {
                break;
            }

            bit_shift += 7;
        }

        let song_string =
            parse_string::<LittleEndian>(&mut cursor, StringLength::Fixed(song_string_length))?;
        let song_data: Vec<&str> = song_string
            .split("\r\n")
            .filter(|s| !s.is_empty())
            .collect();

        if song_data.len() != 2 {
            return Err(TrackReadError::InvalidData {
                name: "song data".to_string(),
                value: song_data.join(","),
            });
        }

        // TODO: Unused
        #[allow(unused_variables)]
        let name = song_data[0];
        #[allow(unused_variables)]
        let seconds_offset = song_data[1].parse::<f64>()?;
    }

    parsed_track.start_position.x = cursor.read_f64::<LittleEndian>()?;
    parsed_track.start_position.y = cursor.read_f64::<LittleEndian>()?;

    let line_count = cursor.read_u32::<LittleEndian>()?;

    let mut max_id = 0;

    for _ in 0..line_count {
        let mut line_id: u32 = 0;
        let flags = cursor.read_u8()?;

        let line_type = match flags & 0x1F {
            1 => LineType::BLUE,
            2 => LineType::RED,
            0 => LineType::GREEN,
            other => {
                return Err(TrackReadError::InvalidData {
                    name: "line type".to_string(),
                    value: other.to_string(),
                });
            }
        };

        let line_inv = (flags >> 7) != 0;
        let line_ext = (flags >> 5) & 0x3;

        let mut line_multiplier: Option<f64> = None;
        let mut line_scenery_width: Option<f64> = None;

        // TODO: Unused
        let mut _line_zoom_target: Option<f32> = None;
        let mut _line_zoom_frames: Option<i16> = None;

        if line_type == LineType::RED && included_features.contains(FEATURE_RED_MULTIPLIER) {
            line_multiplier = Some(cursor.read_u8()? as f64);
        }

        if line_type == LineType::GREEN {
            if included_features.contains(FEATURE_SCENERY_WIDTH) {
                line_scenery_width = Some(cursor.read_u8()? as f64 / 10.0);
            }
        } else {
            if included_features.contains(FEATURE_IGNORABLE_TRIGGER) {
                let has_zoom_trigger = cursor.read_u8()?;
                if has_zoom_trigger == 1 {
                    _line_zoom_target = Some(cursor.read_f32::<LittleEndian>()?);
                    _line_zoom_frames = Some(cursor.read_i16::<LittleEndian>()?);
                }
            }

            line_id = cursor.read_u32::<LittleEndian>()?;
            max_id = max_id.max(line_id);

            if line_ext != 0 {
                _ = cursor.read_i32::<LittleEndian>()?; // Prev line id or -1
                _ = cursor.read_i32::<LittleEndian>()?; // Next line id or -1
            }
        }

        let line_x1 = cursor.read_f64::<LittleEndian>()?;
        let line_y1 = cursor.read_f64::<LittleEndian>()?;
        let line_x2 = cursor.read_f64::<LittleEndian>()?;
        let line_y2 = cursor.read_f64::<LittleEndian>()?;

        let base_line = Line {
            id: line_id,
            x1: line_x1,
            y1: line_y1,
            x2: line_x2,
            y2: line_y2,
            line_type,
        };

        if line_type == LineType::GREEN {
            parsed_track.scenery_lines.push(SceneryLine {
                base_line,
                width: line_scenery_width,
            });
        } else {
            parsed_track.simulation_lines.push(SimulationLine {
                base_line,
                flipped: line_inv,
                left_extension: line_ext & 0x1 != 0,
                right_extension: line_ext & 0x2 != 0,
                multiplier: line_multiplier,
            });
        }
    }

    for line in parsed_track.scenery_lines.iter_mut() {
        max_id += 1;
        line.base_line.id = max_id;
    }

    let current = cursor.stream_position()?;
    let end = cursor.seek(SeekFrom::End(0))?;
    cursor.seek(SeekFrom::Start(current))?;

    // TODO: REMOUNT, ZEROSTART, FRICTIONLESS

    if current == end {
        return Ok(parsed_track);
    }

    // Metadata section

    let mut meta_magic_number = [0u8; 4];
    cursor.read_exact(&mut meta_magic_number)?;

    if &meta_magic_number != b"META" {
        return Err(TrackReadError::InvalidData {
            name: "metadata magic number".to_string(),
            value: bytes_to_hex_string(&meta_magic_number),
        });
    }

    let num_entries = cursor.read_u16::<LittleEndian>()?;

    for _ in 0..num_entries {
        let meta_string = parse_string::<LittleEndian>(&mut cursor, StringLength::U16)?;
        let key_value_pair: Vec<&str> = meta_string.split("=").filter(|s| !s.is_empty()).collect();

        if key_value_pair.len() != 2 {
            return Err(TrackReadError::InvalidData {
                name: "metadata key value pair".to_string(),
                value: key_value_pair.join(","),
            });
        }

        let key = key_value_pair[0];
        let value = key_value_pair[1];

        // TODO: Unused
        match key {
            FEATURE_START_ZOOM => {
                #[allow(unused_variables)]
                let start_zoom = value.parse::<f32>()?;
            }
            FEATURE_X_GRAVITY => {
                #[allow(unused_variables)]
                let x_gravity = value.parse::<f32>()?;
            }
            FEATURE_Y_GRAVITY => {
                #[allow(unused_variables)]
                let y_gravity = value.parse::<f32>()?;
            }
            FEATURE_GRAVITY_WELL_SIZE => {
                #[allow(unused_variables)]
                let gravity_well_size = value.parse::<f64>()?;
            }
            FEATURE_BACKGROUND_COLOR_R => {
                #[allow(unused_variables)]
                let background_color_red = value.parse::<i32>()?;
            }
            FEATURE_BACKGROUND_COLOR_G => {
                #[allow(unused_variables)]
                let background_color_green = value.parse::<i32>()?;
            }
            FEATURE_BACKGROUND_COLOR_B => {
                #[allow(unused_variables)]
                let background_color_blue = value.parse::<i32>()?;
            }
            FEATURE_LINE_COLOR_R => {
                #[allow(unused_variables)]
                let line_color_red = value.parse::<i32>()?;
            }
            FEATURE_LINE_COLOR_G => {
                #[allow(unused_variables)]
                let line_color_green = value.parse::<i32>()?;
            }
            FEATURE_LINE_COLOR_B => {
                #[allow(unused_variables)]
                let line_color_blue = value.parse::<i32>()?;
            }
            FEATURE_TRIGGERS => {
                for (i, trigger) in value.split('&').filter(|s| !s.is_empty()).enumerate() {
                    let values: Vec<&str> = trigger.split(':').filter(|s| !s.is_empty()).collect();

                    if values.len() < 1 {
                        return Err(TrackReadError::InvalidData {
                            name: "size of trigger data".to_string(),
                            value: "0".to_string(),
                        });
                    }

                    match values[0] {
                        "0" => {
                            // Zoom
                            #[allow(unused_variables)]
                            let target_zoom = values[1].parse::<f32>()?;
                            #[allow(unused_variables)]
                            let start_frame = values[2].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let end_frame = values[3].parse::<i32>()?;
                        }
                        "1" => {
                            // Background Color
                            #[allow(unused_variables)]
                            let target_bg_red = values[1].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let target_bg_green = values[2].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let target_bg_blue = values[3].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let start_frame = values[4].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let end_frame = values[5].parse::<i32>()?;
                        }
                        "2" => {
                            // Line Color
                            #[allow(unused_variables)]
                            let target_line_red = values[1].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let target_line_green = values[2].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let target_line_blue = values[3].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let start_frame = values[4].parse::<i32>()?;
                            #[allow(unused_variables)]
                            let end_frame = values[5].parse::<i32>()?;
                        }
                        other => {
                            return Err(TrackReadError::InvalidData {
                                name: format!("triggers {} type", i),
                                value: other.to_string(),
                            });
                        }
                    }
                }
            }
            other => {
                return Err(TrackReadError::InvalidData {
                    name: "metadata key".to_string(),
                    value: other.to_string(),
                });
            }
        }
    }

    // TODO: STARTZOOM, XGRAVITY, YGRAVITY, GRAVITYWELLSIZE, BGCOLORR/G/B, LINECOLORR/G/B, TRIGGERS

    Ok(parsed_track)
}
