pub mod base;
pub mod common;
pub mod reader;
pub mod writer;

pub use reader::read;
pub use writer::write;

use super::InternalTrackFormat;
use anyhow::{Context, Result};
use base::{
    gridver::GRIDVER, label::LABEL, scnline::SCNLINE, simline::SIMLINE, startoffset::STARTOFFSET,
};
use bitflags::bitflags;
use byteorder::{LittleEndian, ReadBytesExt};
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    io::{Cursor, Read},
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ModFlags: u8 {
        const REQUIRED = 1 << 0;
        const PHYSICS = 1 << 1;
        const CAMERA = 1 << 2;
        const SCENERY = 1 << 3;
        const EXTRA_DATA = 1 << 4;
    }
}

#[macro_export]
macro_rules! join_flags {
  ($($flag:ident),+) => {
    crate::lrb::ModFlags::from_bits_truncate($(crate::lrb::ModFlags::$flag.bits() | )+ 0)
  };
}

pub struct ModHandler {
    flags: ModFlags,
    read: Box<dyn Fn(&mut Cursor<&[u8]>, &mut InternalTrackFormat) -> Result<()> + Send + Sync>,
    write: Box<dyn Fn(&mut Cursor<Vec<u8>>, &InternalTrackFormat) -> Result<()> + Send + Sync>,
}

static SUPPORTED_MODS: Lazy<HashMap<(&'static str, u16), &'static Lazy<ModHandler>>> = Lazy::new(|| {
    HashMap::from([
        (("base.gridver", 0), &GRIDVER),
        (("base.label", 0), &LABEL),
        (("base.scnline", 0), &SCNLINE),
        (("base.simline", 0), &SIMLINE),
        (("base.startoffset", 0), &STARTOFFSET),
    ])
});

pub enum StringLength {
    U8,
    U16,
    #[allow(dead_code)]
    U32,
    #[allow(dead_code)]
    Fixed(usize),
}

// Generalized function for reading strings
pub fn parse_string(cursor: &mut Cursor<&[u8]>, length_type: StringLength) -> Result<String> {
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
