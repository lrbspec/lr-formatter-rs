mod amf0;
mod reader;
mod writer;

pub use reader::read;
pub use writer::write;

use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Cursor, Seek};

pub fn get_track_count(data: &[u8]) -> Result<u32> {
    let mut cursor = Cursor::new(data);

    // HACK: We assume header size is constant, and track list length will always be at 0x2C - 0x2F
    cursor.seek(std::io::SeekFrom::Start(0x2C))?;
    let num_tracks = cursor.read_u32::<BigEndian>()?;

    Ok(num_tracks)
}
