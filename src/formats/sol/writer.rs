use std::io::{Cursor, Seek, Write};

use anyhow::Result;
use byteorder::{BigEndian, WriteBytesExt};

use crate::formats::InternalTrackFormat;

pub fn write(internal: &InternalTrackFormat) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());

    cursor.write_all(b"\x00\xBF")?;
    cursor.write_all(b"\x00\x00\x00\x00")?;
    cursor.write_all(b"TCSO")?;
    cursor.write_all(b"\x00\x04\x00\x00\x00\x00")?;
    cursor.write_all(b"\x00\x0AsavedLines")?;
    cursor.write_all(b"\x00\x00\x00\x00")?;
    cursor.write_all(b"\x00\x09trackList")?;

    let file_size = cursor.position() - 6;
    cursor.seek(std::io::SeekFrom::Start(2))?;
    cursor.write_u64::<BigEndian>(file_size)?;

    Ok(cursor.into_inner())
}
