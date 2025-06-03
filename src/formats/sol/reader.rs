use std::io::Cursor;

use anyhow::Result;

use crate::formats::InternalTrackFormat;


pub fn read(data: &[u8]) -> Result<InternalTrackFormat> {
    let mut parsed_track = InternalTrackFormat::filled_default();
    let mut cursor = Cursor::new(data);

    Ok(parsed_track)
}