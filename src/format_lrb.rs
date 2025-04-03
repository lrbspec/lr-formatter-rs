use crate::format_internal::{
    GridVersion, InternalTrackFormat, Line, LineType, SceneryLine, SimulationLine, Vec2,
};
use anyhow::{Context, Result, anyhow};
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::{
    collections::HashMap,
    io::{Cursor, Read, Seek, SeekFrom, Write},
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ModFlags: u8 {
        const OPTIONAL = 1 << 0;
        const PHYSICS = 1 << 1;
        const CAMERA = 1 << 2;
        const SCENERY = 1 << 3;
        const EXTRA_DATA = 1 << 4;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SimLineFlags: u8 {
        const RED = 1 << 0;
        const INVERTED = 1 << 1;
        const LEFT_EXTENSION = 1 << 2;
        const RIGHT_EXTENSION = 1 << 3;
    }
}

macro_rules! join_flags {
    ($($flag:ident),+) => {
        ModFlags::from_bits_truncate($(ModFlags::$flag.bits() | )+ 0)
    };
}

#[derive(Debug)]
struct LRBMod {
    name: &'static str,
    version: u16,
    flags: ModFlags,
    optional_message: Option<&'static str>,
}

const SUPPORTED_MODS: [LRBMod; 5] = [
    LRBMod {
        name: "base.gridver",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, PHYSICS),
        optional_message: Some("specifies grid algorithm (modifies physics)"),
    },
    LRBMod {
        name: "base.label",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA),
        optional_message: Some("contains track name"),
    },
    LRBMod {
        name: "base.scnline",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, SCENERY),
        optional_message: Some("contains scenery lines"),
    },
    LRBMod {
        name: "base.simline",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, PHYSICS, SCENERY),
        optional_message: Some("contains physics lines, affects both physics and visuals"),
    },
    LRBMod {
        name: "base.startoffset",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, PHYSICS),
        optional_message: Some("determines starting position, affects physics"),
    },
];

enum StringLength {
    U8,
    U16,
    #[allow(dead_code)]
    U32,
    #[allow(dead_code)]
    Fixed(usize),
}

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

pub fn parse_lrb(data: &[u8]) -> Result<InternalTrackFormat> {
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

pub fn write_lrb(internal: &InternalTrackFormat) -> Result<Vec<u8>> {
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
