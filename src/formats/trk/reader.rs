use std::{
    collections::HashSet,
    io::{Cursor, Read},
};

use anyhow::{Result, anyhow, bail};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    formats::{
        GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine,
        trk::KNOWN_FEATURES,
    },
    util::{StringLength, parse_string},
};

use super::{
    FEATURE_6_1, FEATURE_IGNORABLE_TRIGGER, FEATURE_RED_MULTIPLIER, FEATURE_SCENERY_WIDTH,
    FEATURE_SONG_INFO, classic_line_type,
};

pub fn read(data: &[u8]) -> Result<InternalTrackFormat> {
    let mut cursor = Cursor::new(data);
    let mut parsed_track = InternalTrackFormat {
        ..Default::default()
    };

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if &magic_number != &[b'T', b'R', b'K', 0xF2] {
        return Err(anyhow!("Read invalid magic number!"));
    }

    // Version
    let version = cursor.read_u8()?;

    if version > 1 {
        return Err(anyhow!("Invalid trk version!"));
    }

    let feature_string = parse_string(&mut cursor, StringLength::U16)?;
    let mut included_features: HashSet<&str> = Default::default();

    for feature in feature_string.split(';') {
        if KNOWN_FEATURES.contains(feature) {
            included_features.insert(feature);
        } else {
            bail!("Came across invalid feature {}!", feature);
        }
    }

    parsed_track.grid_version = if included_features.contains(FEATURE_6_1) {
        GridVersion::V6_1
    } else {
        GridVersion::V6_2
    };

    if included_features.contains(FEATURE_SONG_INFO) {
        let song_string = parse_string(&mut cursor, StringLength::U8)?;
        let song_data: Vec<&str> = song_string.split("\r\n").collect();

        if song_data.len() != 2 {
            bail!("Invalid song data!");
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

    for _ in 0..line_count {
        let mut line_id: u32 = 0;
        let flags = cursor.read_u8()?;
        let line_type = classic_line_type(flags & 0x1F)?;
        let line_inv = (flags >> 7) != 0;
        let line_limit = (flags >> 5) & 0x3;
        let mut line_multiplier: Option<f64> = None;
        let mut line_scenery_width: Option<f64> = None;
        // TODO: Unused
        #[allow(unused_variables)]
        let mut line_zoom_target: Option<f32> = None;
        #[allow(unused_variables)]
        let mut line_zoom_frames: Option<i16> = None;

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
                    line_zoom_target = Some(cursor.read_f32::<LittleEndian>()?);
                    line_zoom_frames = Some(cursor.read_i16::<LittleEndian>()?);
                }
            }

            line_id = cursor.read_u32::<LittleEndian>()?;

            if line_limit != 0 {
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
                left_extension: line_limit & 0x2 != 0,
                right_extension: line_limit & 0x1 != 0,
                multiplier: line_multiplier,
            });
        }
    }

    Ok(parsed_track)
}
