pub mod base;
pub mod common;
pub mod reader;
pub mod writer;

pub use reader::read;
pub use writer::write;

use super::InternalTrackFormat;
use anyhow::Result;
use base::{
    gridver::GRIDVER, label::LABEL, scnline::SCNLINE, simline::SIMLINE, startoffset::STARTOFFSET,
};
use once_cell::sync::Lazy;
use std::{collections::HashMap, io::Cursor};

pub mod mod_flags {
    pub const REQUIRED: u8 = 1 << 0;
    pub const PHYSICS: u8 = 1 << 1;
    pub const CAMERA: u8 = 1 << 2;
    pub const SCENERY: u8 = 1 << 3;
    pub const EXTRA_DATA: u8 = 1 << 4;
}

#[macro_export]
macro_rules! join_flags {
  ($($flag:ident),+) => {
    $(crate::lrb::mod_flags::$flag | )+ 0
  };
}

pub struct ModHandler {
    flags: u8,
    read: Box<dyn Fn(&mut Cursor<&[u8]>, &mut InternalTrackFormat) -> Result<()> + Send + Sync>,
    write: Box<dyn Fn(&mut Cursor<Vec<u8>>, &InternalTrackFormat) -> Result<()> + Send + Sync>,
}

static SUPPORTED_MODS: Lazy<HashMap<(&'static str, u16), &'static Lazy<ModHandler>>> =
    Lazy::new(|| {
        HashMap::from([
            (("base.gridver", 0), &GRIDVER),
            (("base.label", 0), &LABEL),
            (("base.scnline", 0), &SCNLINE),
            (("base.simline", 0), &SIMLINE),
            (("base.startoffset", 0), &STARTOFFSET),
        ])
    });
