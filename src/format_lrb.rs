use std::io::{Read, Write, Cursor};
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use crate::internal::InternalFormat;

pub fn parse_lrb(data: &[u8]) -> Result<InternalFormat, Box<dyn std::error::Error>> {
    let mut cursor = Cursor::new(data);

    // Check magic number
    let mut magic_number = [0u8; 3];
    cursor.read_exact(&mut magic_number)?;
    if &magic_number != b"LRB" {
      return Err("[parse_lrb] Invalid magic number!".into());
    }

    // Read title length and title
    let title_len = cursor.read_u16::<LittleEndian>()? as usize;
    let mut title_bytes = vec![0; title_len];
    cursor.read_exact(&mut title_bytes)?;
    let title = String::from_utf8(title_bytes).unwrap();

    Ok(InternalFormat { title })
}

pub fn write_lrb(internal: &InternalFormat) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    buffer.write_all(b"LRB")?;

    // Write title length and title
    buffer.write_u16::<LittleEndian>(internal.title.len() as u16)?;
    buffer.write_all(internal.title.as_bytes())?;

    Ok(buffer)
}
