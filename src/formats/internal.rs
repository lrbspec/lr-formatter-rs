//! Format that lr_formatter_rs parses into and out of, exposed for usage after reading and before writing

/// Physics grid implementation used, with 6.2 being the default
#[derive(Debug, Clone, Default)]
pub enum GridVersion {
    #[default]
    V6_2 = 0,
    V6_1 = 1,
    V6_0 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    BLUE = 0,
    RED = 1,
    GREEN = 2,
}

#[derive(Debug, Clone, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

/// Base line that all line types derive from
#[derive(Debug, Clone)]
pub struct Line {
    pub id: u32,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub line_type: LineType,
}

#[derive(Debug, Clone)]
pub struct SimulationLine {
    pub base_line: Line,
    pub flipped: bool,
    pub left_extension: bool,
    pub right_extension: bool,
    pub multiplier: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct SceneryLine {
    pub base_line: Line,
    pub width: Option<f64>,
}

/// Struct for storing track properties in an easily accessible way
#[derive(Debug, Clone, Default)]
pub struct InternalTrackFormat {
    pub grid_version: GridVersion,
    pub title: String,
    pub artist: String,
    pub description: String,
    pub duration: u32,
    pub script: String,
    pub simulation_lines: Vec<SimulationLine>,
    pub scenery_lines: Vec<SceneryLine>,
    pub start_position: Vec2,
}

impl InternalTrackFormat {
    /// Creates a new `InternalTrackFormat` with recommended defaults
    pub fn new() -> Self {
        Self {
            duration: 1200,
            ..Default::default()
        }
    }
}
