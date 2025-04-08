use super::{ModFlags, SUPPORTED_MODS, SimLineFlags, StringLength};
use crate::formats::{
    GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2,
};
use anyhow::{Context, Result, anyhow};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

// Generalized function for reading strings
fn parse_string(cursor: &mut Cursor<&[u8]>, length_type: StringLength) -> Result<String> {
    let length = match length_type {
        StringLength::U8 => cursor.read_u8()? as usize,
        StringLength::U16 => cursor.read_u16::<LittleEndian>()? as usize,
        StringLength::U32 => cursor.read_u32::<LittleEndian>()? as usize,
        StringLength::Fixed(size) => size,
    };

    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer).context("Read invalid UTF-8 string")?)
}

pub fn read(data: &[u8]) -> Result<InternalTrackFormat> {
    let mut cursor = Cursor::new(data);
    let mut parsed_track = InternalTrackFormat {
        ..Default::default()
    };

    // Magic number
    let mut magic_number = [0u8; 3];
    cursor.read_exact(&mut magic_number)?;

    if &magic_number != b"LRB" {
        return Err(anyhow!("Read invalid magic number!"));
    }

    // Version
    let _version = cursor.read_u8()?;

    // Number of mods
    let mod_count = cursor.read_u16::<LittleEndian>()?;

    // Mod table
    for _ in 0..mod_count {
        // Name
        let name = parse_string(&mut cursor, StringLength::U8)?;

        // Version
        let version = cursor.read_u16::<LittleEndian>()?;

        println!("[INFO] Loading mod {name} v{version}");

        // Flags
        let flags = ModFlags::from_bits(cursor.read_u8()?).context("Read invalid mod flags!")?;

        let mut offset = 0u64;
        let mut _length = 0u64; // TODO: length is unused

        // Data address
        if flags.contains(ModFlags::EXTRA_DATA) {
            offset = cursor.read_u64::<LittleEndian>()?;
            _length = cursor.read_u64::<LittleEndian>()?;
        }

        let mut optional_message = String::new();
        // Optional message
        if flags.contains(ModFlags::OPTIONAL) {
            optional_message = parse_string(&mut cursor, StringLength::U8)?;
        }

        let supported = SUPPORTED_MODS
            .iter()
            .any(|supported_mod| supported_mod.name == name && supported_mod.version == version);

        if !supported {
            if flags.contains(ModFlags::OPTIONAL) {
                return Err(anyhow!("Required mod {} was not supported!", name));
            }

            println!("[WARNING] This mod is not supported: {optional_message}");
            if flags.contains(ModFlags::SCENERY) {
                println!("[WARNING] Ignoring it may affect scenery rendering.");
            }
            if flags.contains(ModFlags::CAMERA) {
                println!("[WARNING] Ignoring it may affect camera functionality.");
            }
            if flags.contains(ModFlags::PHYSICS) {
                println!("[WARNING] Ignoring it may affect track physics.");
            }
        }

        // We're done if there's no more data
        if !flags.contains(ModFlags::EXTRA_DATA) {
            continue;
        }

        // Record the current position and jump to the extra data position
        let current_position = cursor.stream_position()?;
        cursor.seek(SeekFrom::Start(offset))?;

        match name.as_str() {
            "base.gridver" => {
                let grid_version_number = cursor.read_u8()?;
                parsed_track.grid_version = match grid_version_number {
                    0 => GridVersion::V6_2,
                    1 => GridVersion::V6_1,
                    2 => GridVersion::V6_0,
                    other => Err(anyhow!("Read invalid grid version number {}!", other))?,
                };
            }
            "base.label" => {
                parsed_track.title = parse_string(&mut cursor, StringLength::U16)?;
            }
            "base.scnline" => {
                let num_lines = cursor.read_u32::<LittleEndian>()?;
                for _ in 0..num_lines {
                    let id = cursor.read_u32::<LittleEndian>()?;
                    let x1 = cursor.read_f64::<LittleEndian>()?;
                    let y1 = cursor.read_f64::<LittleEndian>()?;
                    let x2 = cursor.read_f64::<LittleEndian>()?;
                    let y2 = cursor.read_f64::<LittleEndian>()?;
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
            "base.simline" => {
                let num_lines = cursor.read_u32::<LittleEndian>()?;
                for _ in 0..num_lines {
                    let id = cursor.read_u32::<LittleEndian>()?;
                    let line_flags = SimLineFlags::from_bits(cursor.read_u8()?)
                        .context("Read invalid simulation line flags!")?;
                    let x1 = cursor.read_f64::<LittleEndian>()?;
                    let y1 = cursor.read_f64::<LittleEndian>()?;
                    let x2 = cursor.read_f64::<LittleEndian>()?;
                    let y2 = cursor.read_f64::<LittleEndian>()?;
                    let line_type = if line_flags.contains(SimLineFlags::RED) {
                        LineType::RED
                    } else {
                        LineType::BLUE
                    };
                    let flipped = line_flags.contains(SimLineFlags::INVERTED);
                    let left_extension = line_flags.contains(SimLineFlags::LEFT_EXTENSION);
                    let right_extension = line_flags.contains(SimLineFlags::RIGHT_EXTENSION);
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
                        left_extension,
                        right_extension,
                        multiplier: None,
                    });
                }
            }
            "base.startoffset" => {
                let x = cursor.read_f64::<LittleEndian>()?;
                let y = cursor.read_f64::<LittleEndian>()?;
                parsed_track.start_position = Vec2 { x, y };
            }
            other => Err(anyhow!("Came across invalid mod {}!", other))?,
        }

        cursor.seek(SeekFrom::Start(current_position))?;
    }

    Ok(parsed_track)
}
