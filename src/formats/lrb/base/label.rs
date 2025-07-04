use crate::{
    formats::lrb::{ModHandler, mod_flags},
    util::{StringLength, parse_string},
};
use byteorder::{LittleEndian, WriteBytesExt};
use once_cell::sync::Lazy;
use std::io::Write;

// label: u16 length string = the track's label

pub(in crate::formats::lrb) static LABEL: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: mod_flags::EXTRA_DATA,
    read: Box::new(|cursor, output| {
        output.title = parse_string::<LittleEndian>(cursor, StringLength::U16)?;

        Ok(())
    }),
    write: Box::new(|buffer, internal| {
        buffer.write_u16::<LittleEndian>(internal.title.len() as u16)?;
        buffer.write_all(internal.title.as_bytes())?;

        Ok(())
    }),
});
