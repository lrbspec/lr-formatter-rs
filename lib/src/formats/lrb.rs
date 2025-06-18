mod base;
mod common;
mod reader;
mod writer;

pub use reader::read;
pub use writer::write;

use crate::formats::internal::InternalTrackFormat;
use anyhow::Result;
use base::{GRIDVER, LABEL, SCNLINE, SIMLINE, STARTOFFSET};
use once_cell::sync::Lazy;
use std::{collections::HashMap, io::Cursor};

mod mod_flags {
    pub(crate) const REQUIRED: u8 = 1 << 0;
    pub(crate) const PHYSICS: u8 = 1 << 1;
    pub(crate) const CAMERA: u8 = 1 << 2;
    pub(crate) const SCENERY: u8 = 1 << 3;
    pub(crate) const EXTRA_DATA: u8 = 1 << 4;
}

struct ModHandler {
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
