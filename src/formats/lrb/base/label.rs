use crate::{
    formats::lrb::{ModHandler, StringLength, parse_string},
    join_flags,
};
use byteorder::{LittleEndian, WriteBytesExt};
use once_cell::sync::Lazy;
use std::io::Write;

// label: u16 length string = the track's label

pub static LABEL: Lazy<ModHandler> = Lazy::new(|| ModHandler {
    flags: join_flags!(EXTRA_DATA),
    read: Box::new(|cursor, output| {
        output.title = parse_string(cursor, StringLength::U16)?;

        Ok(())
    }),
    write: Box::new(|buffer, internal| {
        buffer.write_u16::<LittleEndian>(internal.title.len() as u16)?;
        buffer.write_all(internal.title.as_bytes())?;

        Ok(())
    }),
});
