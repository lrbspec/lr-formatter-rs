pub mod base;
pub mod common;
pub mod reader;
pub mod writer;

use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ModFlags: u8 {
        const OPTIONAL = 1 << 0;
        const PHYSICS = 1 << 1;
        const CAMERA = 1 << 2;
        const SCENERY = 1 << 3;
        const EXTRA_DATA = 1 << 4;
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SimLineFlags: u8 {
        const RED = 1 << 0;
        const INVERTED = 1 << 1;
        const LEFT_EXTENSION = 1 << 2;
        const RIGHT_EXTENSION = 1 << 3;
    }
}

macro_rules! join_flags {
    ($($flag:ident),+) => {
        ModFlags::from_bits_truncate($(ModFlags::$flag.bits() | )+ 0)
    };
}

#[derive(Debug)]
struct LRBMod {
    name: &'static str,
    version: u16,
    flags: ModFlags,
    optional_message: Option<&'static str>,
}

const SUPPORTED_MODS: [LRBMod; 5] = [
    LRBMod {
        name: "base.gridver",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, PHYSICS),
        optional_message: Some("specifies grid algorithm (modifies physics)"),
    },
    LRBMod {
        name: "base.label",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA),
        optional_message: Some("contains track name"),
    },
    LRBMod {
        name: "base.scnline",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, SCENERY),
        optional_message: Some("contains scenery lines"),
    },
    LRBMod {
        name: "base.simline",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, PHYSICS, SCENERY),
        optional_message: Some("contains physics lines, affects both physics and visuals"),
    },
    LRBMod {
        name: "base.startoffset",
        version: 0,
        flags: join_flags!(OPTIONAL, EXTRA_DATA, PHYSICS),
        optional_message: Some("determines starting position, affects physics"),
    },
];

enum StringLength {
    U8,
    U16,
    #[allow(dead_code)]
    U32,
    #[allow(dead_code)]
    Fixed(usize),
}
