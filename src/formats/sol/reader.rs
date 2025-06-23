use std::io::{Cursor, Read};

use anyhow::{Context, Result, bail};
use byteorder::{BigEndian, ReadBytesExt};

use crate::{
    formats::{
        internal::{GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine},
        sol::amf0::deserialize,
    },
    util::{StringLength, parse_string},
};

pub fn read(data: &[u8], track_index: Option<u32>) -> Result<InternalTrackFormat> {
    let mut parsed_track = InternalTrackFormat::new();
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 2];
    cursor.read_exact(&mut magic_number)?;

    if &magic_number != &[0x00, 0xBF] {
        bail!("Read invalid magic number!");
    }

    // Header
    let _file_size = cursor.read_u32::<BigEndian>()? + 6;

    let mut tag = [0u8; 4];
    cursor.read_exact(&mut tag)?;

    if &tag != &[b'T', b'C', b'S', b'O'] {
        bail!("Read invalid tag {:?}!", tag);
    }

    let mut marker = [0u8; 6];
    cursor.read_exact(&mut marker)?;
    if &marker != &[0x00, 0x04, 0x00, 0x00, 0x00, 0x00] {
        bail!("Read invalid marker {:?}!", marker);
    }

    let sol_name = parse_string::<BigEndian>(&mut cursor, StringLength::U16)?;
    if sol_name.as_str() != "savedLines" {
        bail!("Read invalid SOL name {}!", sol_name);
    }

    let _padding = cursor.read_u32::<BigEndian>()?;

    let data_name = parse_string::<BigEndian>(&mut cursor, StringLength::U16)?;
    if data_name.as_str() != "trackList" {
        bail!("Read invalid data name {}!", data_name);
    }

    // Track Data
    let current_pos = cursor.position() as usize;
    let trimmed = &data[current_pos..data.len().saturating_sub(1)]; // trim off the last byte \x00
    cursor = Cursor::new(trimmed);

    let result = &deserialize(&mut cursor).expect("Failed to deserialize AMF0 format!");
    let track_list_amf = &result[0];
    let track_list = track_list_amf
        .clone()
        .get_object_properties()
        .context("Track list was not valid object!")?;

    let target_track_index = match track_index {
        Some(index) => &index.to_string(),
        None => "0",
    };

    let target_track_amf = track_list
        .get(target_track_index)
        .context("Invalid track index supplied!")?;

    let target_track = target_track_amf
        .clone()
        .get_object_properties()
        .context("Track was not valid object!")?;

    if let Some(val) = target_track.get("label") {
        parsed_track.title = val
            .clone()
            .get_string()
            .context("Label was not valid string!")?;
    }

    if let Some(val) = target_track.get("version") {
        let version_string = val
            .clone()
            .get_string()
            .context("Label was not valid string!")?;
        parsed_track.grid_version = match version_string.as_str() {
            "6.0" => GridVersion::V6_0,
            "6.1" => GridVersion::V6_1,
            "6.2" => GridVersion::V6_2,
            _ => bail!("Invalid grid version {}!", version_string),
        }
    } else {
        parsed_track.grid_version = GridVersion::V6_0
    }

    if let Some(val) = target_track.get("startLine") {
        let start_position = val
            .clone()
            .get_object_properties()
            .context("Start line was not valid object!")?;

        let start_x_amf = start_position
            .get("0")
            .context("Start line did not have prop '0'!")?;
        parsed_track.start_position.x = start_x_amf
            .clone()
            .get_number()
            .context("Start line did not have valid x position!")?;

        let start_y_amf = start_position
            .get("1")
            .context("Start line did not have prop '1'!")?;
        parsed_track.start_position.y = start_y_amf
            .clone()
            .get_number()
            .context("Start line did not have valid y position!")?;
    }

    if let Some(_) = target_track.get("trackData") {
        // TODO: Enable zero start (don't bother parsing)
    }

    if let Some(val) = target_track.get("data") {
        let lines_list = val
            .clone()
            .get_object_properties()
            .context("Lines list was not valid object!")?;

        for index in lines_list.keys() {
            let line_amf = lines_list.get(index).context("Line was somehow None!")?;
            let line = line_amf
                .clone()
                .get_object_properties()
                .context("Line was not valid object!")?;

            let x1_amf = line.get("0").context("Line did not have '0' prop!")?;
            let x1 = x1_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'x1' prop!")?;

            let y1_amf = line.get("1").context("Line did not have '1' prop!")?;
            let y1 = y1_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'y1' prop!")?;

            let x2_amf = line.get("2").context("Line did not have '2' prop!")?;
            let x2 = x2_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'x2' prop!")?;

            let y2_amf = line.get("3").context("Line did not have '3' prop!")?;
            let y2 = y2_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'y2' prop!")?;

            let ext_amf = line.get("4").context("Line did not have '4' prop!")?;
            let ext = ext_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'ext' prop!")?;
            let left_extension = ext == 1.0 || ext == 3.0;
            let right_extension = ext == 2.0 || ext == 3.0;

            let flipped_amf = line.get("5").context("Line did not have '5' prop!")?;
            let flipped = flipped_amf
                .clone()
                .get_boolean()
                .or_else(|| flipped_amf.clone().get_number().map(|num| num == 1.0))
                .context("Line did not have valid 'flipped' prop!")?;

            let id_amf = line.get("8").context("Line did not have '8' prop!")?;
            let id_float = id_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'id' prop!")?;
            let unsafe_id =
                if id_float.is_finite() && id_float >= 0.0 && id_float <= u32::MAX as f64 {
                    Some(id_float as u32)
                } else {
                    None
                };
            let id = match unsafe_id {
                Some(val) => val,
                None => bail!("Invalid line id {}!", id_float),
            };

            let line_type_amf = line.get("9").context("Line did not have '9' prop!")?;
            let line_type_numeric = line_type_amf
                .clone()
                .get_number()
                .context("Line did not have valid 'type' prop!")?;
            let line_type = match line_type_numeric {
                0.0 => LineType::BLUE,
                1.0 => LineType::RED,
                2.0 => LineType::GREEN,
                _ => bail!("Invalid line type {}!", line_type_numeric),
            };

            let base_line = Line {
                x1,
                y1,
                x2,
                y2,
                id,
                line_type,
            };

            if line_type == LineType::GREEN {
                parsed_track.scenery_lines.push(SceneryLine {
                    base_line,
                    width: None,
                });
            } else {
                parsed_track.simulation_lines.push(SimulationLine {
                    base_line,
                    flipped,
                    left_extension,
                    right_extension,
                    multiplier: None,
                });
            }
        }
    }

    Ok(parsed_track)
}
