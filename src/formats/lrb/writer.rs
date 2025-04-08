use super::{ModFlags, SUPPORTED_MODS, SimLineFlags};
use crate::formats::internal::{GridVersion, InternalTrackFormat, LineType};
use anyhow::{Result, anyhow};
use byteorder::{LittleEndian, WriteBytesExt};
use std::{
    collections::HashMap,
    io::{Cursor, Seek, SeekFrom, Write},
};

pub fn write(internal: &InternalTrackFormat) -> Result<Vec<u8>> {
    let mut buffer = Cursor::new(Vec::new());
    let mut mod_table_entry_offsets: HashMap<String, u64> = HashMap::new();

    // Magic number
    buffer.write_all(b"LRB")?;

    // Version
    buffer.write_u8(0)?;

    // Number of mods
    buffer.write_u16::<LittleEndian>(5)?;

    // Mod table
    for supported_mod in SUPPORTED_MODS {
        // Name
        buffer.write_u8(supported_mod.name.len().try_into()?)?;
        buffer.write_all(supported_mod.name.as_bytes())?;

        // Version
        buffer.write_u16::<LittleEndian>(supported_mod.version)?;

        // Flags
        buffer.write_u8(supported_mod.flags.bits())?;

        // Data address
        if supported_mod.flags.contains(ModFlags::EXTRA_DATA) {
            mod_table_entry_offsets
                .insert(supported_mod.name.to_string(), buffer.stream_position()?);

            // Allocate space for data address information
            buffer.write_u64::<LittleEndian>(0)?;
            buffer.write_u64::<LittleEndian>(0)?;
        }

        // Optional message
        if supported_mod.flags.contains(ModFlags::OPTIONAL) {
            buffer.write_u8(supported_mod.optional_message.unwrap().len().try_into()?)?;
            buffer.write_all(supported_mod.optional_message.unwrap().as_bytes())?;
        }
    }

    for supported_mod in SUPPORTED_MODS {
        let section_start = buffer.stream_position()?;

        match supported_mod.name {
            "base.gridver" => {
                // grid version: u8 = the grid algorithm version used by the track

                let version_number = match internal.grid_version {
                    GridVersion::V6_0 => 2,
                    GridVersion::V6_1 => 1,
                    GridVersion::V6_2 => 0,
                };
                buffer.write_u8(version_number)?;
            }
            "base.label" => {
                // label: u16 length string = the track's label
                buffer.write_u16::<LittleEndian>(internal.title.len().try_into()?)?;
                buffer.write_all(internal.title.as_bytes())?;
            }
            "base.scnline" => {
                // count: u32 = the amount of lines written
                // lines: scnline[count] = [
                //   id: u32 = the line's ID
                //   x1: f64 = the x position of the 1st point
                //   y1: f64 = the y position of the 1st point
                //   x2: f64 = the x position of the 2nd point
                //   y2: f64 = the y position of the 2nd point
                // ]

                buffer.write_u32::<LittleEndian>(internal.scenery_lines.len().try_into()?)?;

                for scenery_line in &internal.scenery_lines {
                    buffer.write_u32::<LittleEndian>(scenery_line.base_line.id)?;
                    buffer.write_f64::<LittleEndian>(scenery_line.base_line.x1)?;
                    buffer.write_f64::<LittleEndian>(scenery_line.base_line.y1)?;
                    buffer.write_f64::<LittleEndian>(scenery_line.base_line.x2)?;
                    buffer.write_f64::<LittleEndian>(scenery_line.base_line.y2)?;
                }
            }
            "base.simline" => {
                // count: u32 = the amount of lines written
                // lines: simline[count] = [
                //   id: u32 = the line's ID
                //   flags: u8 = Line flags 0000DCBA
                //   x1: f64 = the x position of the 1st point
                //   y1: f64 = the y position of the 1st point
                //   x2: f64 = the x position of the 2nd point
                //   y2: f64 = the y position of the 2nd point
                // ]
                // Line flag defs: A = Red line, B = inverted, C = left extension, D = right extension

                buffer.write_u32::<LittleEndian>(internal.simulation_lines.len().try_into()?)?;

                for simulation_line in &internal.simulation_lines {
                    let mut line_flags = SimLineFlags::empty();
                    if simulation_line.base_line.line_type == LineType::RED {
                        line_flags.insert(SimLineFlags::RED);
                    }
                    if simulation_line.flipped {
                        line_flags.insert(SimLineFlags::INVERTED);
                    }
                    if simulation_line.left_extension {
                        line_flags.insert(SimLineFlags::LEFT_EXTENSION);
                    }
                    if simulation_line.right_extension {
                        line_flags.insert(SimLineFlags::RIGHT_EXTENSION);
                    }
                    buffer.write_u32::<LittleEndian>(simulation_line.base_line.id)?;
                    buffer.write_u8(line_flags.bits())?;
                    buffer.write_f64::<LittleEndian>(simulation_line.base_line.x1)?;
                    buffer.write_f64::<LittleEndian>(simulation_line.base_line.y1)?;
                    buffer.write_f64::<LittleEndian>(simulation_line.base_line.x2)?;
                    buffer.write_f64::<LittleEndian>(simulation_line.base_line.y2)?;
                }
            }
            "base.startoffset" => {
                // X: f64 = the X coordinate of the start offset
                // Y: f64 = the Y coordinate of the start offset (remember +Y is down)

                buffer.write_f64::<LittleEndian>(internal.start_position.x)?;
                buffer.write_f64::<LittleEndian>(internal.start_position.y)?;
            }
            other => Err(anyhow!("Implementation not written for {}!", other))?,
        }

        let section_end = buffer.stream_position()?;
        let section_length = section_end - section_start;

        if let Some(&offset) = mod_table_entry_offsets.get(supported_mod.name) {
            buffer.seek(SeekFrom::Start(offset))?;
            buffer.write_u64::<LittleEndian>(section_start)?;
            buffer.write_u64::<LittleEndian>(section_length)?;
            buffer.seek(SeekFrom::Start(section_end))?;
        }
    }

    Ok(buffer.into_inner())
}
