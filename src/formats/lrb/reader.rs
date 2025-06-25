use super::{SUPPORTED_MODS, mod_flags};
use crate::{
    TrackReadError,
    formats::internal::InternalTrackFormat,
    util::{self, StringLength, parse_string},
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::io::{Cursor, Read, Seek, SeekFrom};

pub fn read(data: &[u8]) -> Result<InternalTrackFormat, TrackReadError> {
    let mut internal = InternalTrackFormat::new();
    let mut cursor = Cursor::new(data);

    // Magic number
    let mut magic_number = [0u8; 3];
    cursor.read_exact(&mut magic_number)?;

    if &magic_number != b"LRB" {
        return Err(TrackReadError::InvalidData {
            name: "magic_number".to_string(),
            value: util::bytes_to_hex_string(&magic_number),
        });
    }

    // Version
    let _version = cursor.read_u8()?;

    // Number of mods
    let mod_count = cursor.read_u16::<LittleEndian>()?;

    // Mod table
    for _ in 0..mod_count {
        // Name
        let name = parse_string::<LittleEndian>(&mut cursor, StringLength::U8)?;

        // Version
        let version = cursor.read_u16::<LittleEndian>()?;

        // Flags
        let flags = cursor.read_u8()?;

        let mut offset = 0u64;
        // TODO: length is unused (is it necessary?)
        let mut _length = 0u64;

        // Data address
        if flags & mod_flags::EXTRA_DATA != 0 {
            offset = cursor.read_u64::<LittleEndian>()?;
            _length = cursor.read_u64::<LittleEndian>()?;
        }

        if !SUPPORTED_MODS.contains_key(&(name.as_str(), version)) {
            if flags & mod_flags::REQUIRED != 0 {
                return Err(TrackReadError::Other {
                    message: format!("Required mod not supported: {} v{}", name, version),
                });
            }

            // TODO: Include these flags in the internal format (how?)
            // println!("[WARNING] This mod is not supported: {} v{}", name, version);

            if flags & mod_flags::SCENERY != 0 {
                // println!("Ignoring it may affect scenery rendering.");
            }
            if flags & mod_flags::CAMERA != 0 {
                // println!("Ignoring it may affect camera functionality.");
            }
            if flags & mod_flags::PHYSICS != 0 {
                // println!("Ignoring it may affect track physics.");
            }
        }

        // We're done if there's no more data
        if flags & mod_flags::EXTRA_DATA == 0 {
            continue;
        }

        // Record the current position and jump to the extra data position
        let current_position = cursor.stream_position()?;
        cursor.seek(SeekFrom::Start(offset))?;

        let mod_identifier = (name.as_str(), version);
        match SUPPORTED_MODS.get(&mod_identifier) {
            Some(mod_handler) => {
                (mod_handler.read)(&mut cursor, &mut internal)?;
            }
            None => {
                return Err(TrackReadError::InvalidData {
                    name: "mod_identifier".to_string(),
                    value: format!("{} v{}", mod_identifier.0, mod_identifier.1),
                });
            }
        }

        cursor.seek(SeekFrom::Start(current_position))?;
    }

    Ok(internal)
}
