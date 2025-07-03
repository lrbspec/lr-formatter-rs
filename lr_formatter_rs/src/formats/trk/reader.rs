use std::{
    collections::HashSet,
    io::{Cursor, Read, Seek, SeekFrom},
};

use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    formats::{
        TrackReadError,
        trk::{
            FEATURE_BACKGROUND_COLOR_B, FEATURE_BACKGROUND_COLOR_G, FEATURE_BACKGROUND_COLOR_R,
            FEATURE_FRICTIONLESS, FEATURE_GRAVITY_WELL_SIZE, FEATURE_LINE_COLOR_B,
            FEATURE_LINE_COLOR_G, FEATURE_REMOUNT, FEATURE_START_ZOOM, FEATURE_TRIGGERS,
            FEATURE_X_GRAVITY, FEATURE_Y_GRAVITY, FEATURE_ZERO_START,
        },
    },
    track::{
        GridVersion, LineType, RGBColor, Track, TrackBuilder, Vec2, line::line_group::LineFeature,
    },
    util::{StringLength, bytes_to_hex_string, parse_string},
};

use super::{
    FEATURE_6_1, FEATURE_IGNORABLE_TRIGGER, FEATURE_LINE_COLOR_R, FEATURE_RED_MULTIPLIER,
    FEATURE_SCENERY_WIDTH, FEATURE_SONG_INFO,
};

pub fn read(data: Vec<u8>) -> Result<Track, TrackReadError> {
    let track_builder = &mut TrackBuilder::new();

    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if magic_number != [b'T', b'R', b'K', 0xF2] {
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
        included_features.insert(feature);
        if feature == FEATURE_RED_MULTIPLIER {
            track_builder
                .line_group()
                .enable_feature(LineFeature::AccelerationMultiplier);
        }
        if feature == FEATURE_SCENERY_WIDTH {
            track_builder
                .line_group()
                .enable_feature(LineFeature::SceneryWidth);
        }
        // TODO: Attach warning if feature not accounted for
    }

    let grid_version = if included_features.contains(FEATURE_6_1) {
        GridVersion::V6_1
    } else {
        GridVersion::V6_2
    };
    track_builder.metadata().grid_version(grid_version);

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

        let name = song_data[0];
        let seconds_offset = song_data[1].parse::<f64>()?;
        track_builder
            .metadata()
            .audio_filename(name)
            .audio_offset_until_start(-seconds_offset);
    }

    let start_pos_x = cursor.read_f64::<LittleEndian>()?;
    let start_pos_y = cursor.read_f64::<LittleEndian>()?;
    track_builder.metadata().start_position(Vec2 {
        x: start_pos_x,
        y: start_pos_y,
    });

    let line_count = cursor.read_u32::<LittleEndian>()?;

    let mut max_id = 0;

    for _ in 0..line_count {
        let mut line_id: u32 = 0;
        let flags = cursor.read_u8()?;

        let line_type = match flags & 0x1F {
            1 => LineType::Standard,
            2 => LineType::Acceleration,
            0 => LineType::Scenery,
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

        let mut line_zoom_target: Option<f32> = None;
        let mut line_zoom_frames: Option<i16> = None;

        if line_type == LineType::Acceleration && included_features.contains(FEATURE_RED_MULTIPLIER)
        {
            line_multiplier = Some(cursor.read_u8()? as f64);
        }

        if line_type == LineType::Scenery {
            if included_features.contains(FEATURE_SCENERY_WIDTH) {
                line_scenery_width = Some(cursor.read_u8()? as f64 / 10.0);
            }
        } else {
            if included_features.contains(FEATURE_IGNORABLE_TRIGGER) {
                let has_zoom_trigger = cursor.read_u8()?;
                if has_zoom_trigger == 1 {
                    line_zoom_target = Some(cursor.read_f32::<LittleEndian>()?);
                    line_zoom_frames = Some(cursor.read_i16::<LittleEndian>()?);
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
        let endpoints = (
            Vec2 {
                x: line_x1,
                y: line_y1,
            },
            Vec2 {
                x: line_x2,
                y: line_y2,
            },
        );
        let left_ext = line_ext & 0x1 != 0;
        let right_ext = line_ext & 0x2 != 0;

        match line_type {
            LineType::Standard => {
                track_builder
                    .line_group()
                    .add_standard_line(line_id, endpoints, line_inv, left_ext, right_ext)?;
            }
            LineType::Acceleration => {
                track_builder
                    .line_group()
                    .add_acceleration_line(line_id, endpoints, line_inv, left_ext, right_ext)?
                    .multiplier(line_multiplier);
            }
            LineType::Scenery => {
                track_builder
                    .line_group()
                    .add_scenery_line(line_id, endpoints)?
                    .width(line_scenery_width);
            }
        }
    }

    for line in track_builder.line_group().get_scenery_lines() {
        max_id += 1;
        line.id(max_id);
    }

    track_builder
        .metadata()
        .zero_friction_riders(included_features.contains(FEATURE_FRICTIONLESS));

    if included_features.contains(FEATURE_REMOUNT) {
        track_builder.metadata().remount(true);
        // TODO: Should this also be set?
        track_builder.metadata().use_legacy_remount(true);
    }

    if included_features.contains(FEATURE_ZERO_START) {
        track_builder.metadata().zero_start(true);
    }

    let current = cursor.stream_position()?;
    let end = cursor.seek(SeekFrom::End(0))?;
    cursor.seek(SeekFrom::Start(current))?;

    if current == end {
        return Ok(track_builder.build()?);
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

    let mut start_zoom = 4.0;
    let mut start_gravity = Vec2 { x: 0.0, y: 1.0 };
    let mut gravity_well_size = 10.0;
    let mut start_background_color = RGBColor {
        red: 244,
        green: 245,
        blue: 249,
    };
    let mut start_line_color = RGBColor {
        red: 0,
        green: 0,
        blue: 0,
    };

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

        match key {
            FEATURE_START_ZOOM => {
                start_zoom = value.parse::<f32>()? as f64;
            }
            FEATURE_X_GRAVITY => {
                start_gravity.x = value.parse::<f32>()? as f64;
            }
            FEATURE_Y_GRAVITY => {
                start_gravity.y = value.parse::<f32>()? as f64;
            }
            FEATURE_GRAVITY_WELL_SIZE => {
                gravity_well_size = value.parse::<f64>()?;
            }
            FEATURE_BACKGROUND_COLOR_R => {
                start_background_color.red = value.parse::<i32>()? as u8;
            }
            FEATURE_BACKGROUND_COLOR_G => {
                start_background_color.green = value.parse::<i32>()? as u8;
            }
            FEATURE_BACKGROUND_COLOR_B => {
                start_background_color.blue = value.parse::<i32>()? as u8;
            }
            FEATURE_LINE_COLOR_R => {
                start_line_color.red = value.parse::<i32>()? as u8;
            }
            FEATURE_LINE_COLOR_G => {
                start_line_color.green = value.parse::<i32>()? as u8;
            }
            FEATURE_LINE_COLOR_B => {
                start_line_color.blue = value.parse::<i32>()? as u8;
            }
            FEATURE_TRIGGERS => {
                for (i, trigger) in value.split('&').filter(|s| !s.is_empty()).enumerate() {
                    let values: Vec<&str> = trigger.split(':').filter(|s| !s.is_empty()).collect();

                    if values.is_empty() {
                        return Err(TrackReadError::InvalidData {
                            name: "size of trigger data".to_string(),
                            value: "0".to_string(),
                        });
                    }

                    match values[0] {
                        "0" => {
                            // Zoom
                            let target_zoom = values[1].parse::<f32>()?;
                            let start_frame = values[2].parse::<i32>()?;
                            let end_frame = values[3].parse::<i32>()?;
                        }
                        "1" => {
                            // Background Color
                            let target_bg_red = values[1].parse::<i32>()?;
                            let target_bg_green = values[2].parse::<i32>()?;
                            let target_bg_blue = values[3].parse::<i32>()?;
                            let start_frame = values[4].parse::<i32>()?;
                            let end_frame = values[5].parse::<i32>()?;
                        }
                        "2" => {
                            // Line Color
                            let target_line_red = values[1].parse::<i32>()?;
                            let target_line_green = values[2].parse::<i32>()?;
                            let target_line_blue = values[3].parse::<i32>()?;
                            let start_frame = values[4].parse::<i32>()?;
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

    track_builder.metadata().start_zoom(start_zoom);
    track_builder.metadata().start_gravity(start_gravity);
    track_builder
        .metadata()
        .gravity_well_size(gravity_well_size);
    track_builder
        .metadata()
        .start_background_color(start_background_color);
    track_builder.metadata().start_line_color(start_line_color);

    Ok(track_builder.build()?)
}
