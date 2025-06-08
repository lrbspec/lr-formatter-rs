use std::io::{Cursor, Read, Seek, SeekFrom};

use anyhow::{Result, anyhow, bail};
use byteorder::{LittleEndian, ReadBytesExt};

use crate::{
    formats::{
        GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine,
        lrpk::{
            LUMP_PHYSICS_LINES, LUMP_RIDER_PROPERTIES, LUMP_SCENERY_LINES, LUMP_TRACK_PROPERTIES,
            LUMP_VERSION_INFO,
        },
    },
    util::{StringLength, parse_string},
};

pub fn read(data: &[u8]) -> Result<InternalTrackFormat> {
    let mut parsed_track = InternalTrackFormat::filled_default();
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 4];
    cursor.read_exact(&mut magic_number)?;

    if &magic_number != &[b'L', b'R', b'P', b'K'] {
        return Err(anyhow!("Read invalid magic number!"));
    }

    // Directory list information
    let num_directories = cursor.read_u32::<LittleEndian>()?;
    let directory_list_loc = cursor.read_u32::<LittleEndian>()?;

    cursor.seek(SeekFrom::Start(u64::from(directory_list_loc)))?;

    for i in 0..num_directories {
        let lump = parse_string::<LittleEndian>(&mut cursor, StringLength::Fixed(8))?;
        let lump_str = lump.as_str();
        let lump_offset = u64::from(cursor.read_u32::<LittleEndian>()?);

        let current_position = cursor.stream_position()?;
        cursor.seek(SeekFrom::Start(lump_offset))?;

        if i == 0 && lump_str != LUMP_VERSION_INFO {
            bail!("First lump should be {}!", LUMP_VERSION_INFO)
        }

        match lump_str {
            LUMP_VERSION_INFO => {
                let indev = cursor.read_u8()? == 1;

                if indev {
                    bail!("Indev LRPK not supported!")
                }

                // May not need to parse, since neither .com nor LRA ended up
                // implementing this save format
                let _save_source = cursor.read_u8()?;

                let lib_version = cursor.read_u8()?;
                println!("Made with HXLR version {}", lib_version);

                let save_revision = cursor.read_u8()?;
                if save_revision != 1 {
                    bail!("Only save revision 1 supported!")
                }

                // Engine specific, can ignore
                let _source_port_revision = cursor.read_u8()?;
            }
            LUMP_PHYSICS_LINES => {
                let num_lines = cursor.read_u32::<LittleEndian>()?;
                for _ in 0..num_lines {
                    let id = cursor.read_u32::<LittleEndian>()?;
                    let x1 = cursor.read_f64::<LittleEndian>()?;
                    let y1 = cursor.read_f64::<LittleEndian>()?;
                    let x2 = cursor.read_f64::<LittleEndian>()?;
                    let y2 = cursor.read_f64::<LittleEndian>()?;
                    let numeric_line_type = cursor.read_u8()?;
                    let flipped = cursor.read_u8()? == 1;
                    let line_ext = cursor.read_u8()?;

                    let line_type = match numeric_line_type {
                        0 => LineType::BLUE,
                        1 => LineType::RED,
                        // TODO other line types
                        _ => bail!("Unknown physics line type {}!", numeric_line_type),
                    };

                    let base_line = Line {
                        id,
                        x1,
                        y1,
                        x2,
                        y2,
                        line_type,
                    };

                    parsed_track.simulation_lines.push(SimulationLine {
                        base_line,
                        flipped,
                        left_extension: line_ext & 0x1 != 0,
                        right_extension: line_ext & 0x2 != 0,
                        multiplier: None,
                    });
                }
            }
            LUMP_SCENERY_LINES => {
                let num_lines = cursor.read_u32::<LittleEndian>()?;
                for _ in 0..num_lines {
                    let id = cursor.read_u32::<LittleEndian>()?;
                    let x1 = f64::from(cursor.read_f32::<LittleEndian>()?);
                    let y1 = f64::from(cursor.read_f32::<LittleEndian>()?);
                    let x2 = f64::from(cursor.read_f32::<LittleEndian>()?);
                    let y2 = f64::from(cursor.read_f32::<LittleEndian>()?);

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
            LUMP_RIDER_PROPERTIES => {
                let x_start = cursor.read_f64::<LittleEndian>()?;
                let y_start = cursor.read_f64::<LittleEndian>()?;
                // TODO: Each lump adds a new rider, for now just adjust start point
                parsed_track.start_position.x = x_start;
                parsed_track.start_position.y = y_start;
            }
            LUMP_TRACK_PROPERTIES => {
                let track_name = parse_string::<LittleEndian>(&mut cursor, StringLength::U8)?;
                let author_name = parse_string::<LittleEndian>(&mut cursor, StringLength::U8)?;
                let numeric_grid_type = cursor.read_u8()?;

                parsed_track.title = track_name;
                parsed_track.artist = author_name;
                parsed_track.grid_version = match numeric_grid_type {
                    0 => GridVersion::V6_0,
                    1 => GridVersion::V6_1,
                    2 => GridVersion::V6_2,
                    _ => bail!("Invalid grid version {}!", numeric_grid_type),
                }
            }
            _ => bail!("Came across unknown lump {}!", lump_str),
        }

        cursor.seek(SeekFrom::Start(current_position))?;
    }

    Ok(parsed_track)
}
