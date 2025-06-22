use anyhow::{Context, Result};
use byteorder::{ByteOrder, ReadBytesExt};
use std::io::{Cursor, Read};

pub(crate) enum StringLength {
    U8,
    U16,
    #[allow(dead_code)]
    U32,
    #[allow(dead_code)]
    Fixed(usize),
}

// Generalized function for reading strings
pub(crate) fn parse_string<B: ByteOrder>(
    cursor: &mut Cursor<&[u8]>,
    length_type: StringLength,
) -> Result<String> {
    let length = match length_type {
        StringLength::U8 => cursor.read_u8()? as usize,
        StringLength::U16 => cursor.read_u16::<B>()? as usize,
        StringLength::U32 => cursor.read_u32::<B>()? as usize,
        StringLength::Fixed(size) => size,
    };

    let mut buffer = vec![0; length];
    cursor.read_exact(&mut buffer)?;

    Ok(String::from_utf8(buffer).context("Read invalid UTF-8 string")?)
}
