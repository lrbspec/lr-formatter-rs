use std::io::{Cursor, Write};

use anyhow::Result;

use crate::formats::InternalTrackFormat;



pub fn write(internal: &InternalTrackFormat) -> Result<Vec<u8>> {
    let mut cursor = Cursor::new(Vec::new());
    cursor.write_all(b"LRB")?;

    Ok(cursor.into_inner())
}