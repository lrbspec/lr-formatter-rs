pub mod reader;

pub use reader::read;

const LUMP_VERSION_INFO: &str = "VERSINFO";
const LUMP_TRACK_PROPERTIES: &str = "TRACKDEF";
const LUMP_PHYSICS_LINES: &str = "LINEDEF ";
const LUMP_SCENERY_LINES: &str = "LINEDECO";
const LUMP_RIDER_PROPERTIES: &str = "RIDERDEF";
