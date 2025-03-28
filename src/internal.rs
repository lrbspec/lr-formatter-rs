#[derive(Debug, Clone)]
pub enum GridVersion {
    V6_2 = 0,
    V6_1 = 1,
    V6_0 = 2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LineType {
    BLUE = 0,
    RED = 1,
    GREEN = 2,
}

// TODO: make struct attribute access more well defined by removing public access
#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

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
    pub multiplier: Option<f64>, // TODO: stored within a byte in older LRA builds
}

#[derive(Debug, Clone)]
pub struct SceneryLine {
    pub base_line: Line,
    pub width: Option<f64>, // TODO: stored within a byte in older LRA builds
}

#[derive(Debug, Clone)]
pub struct InternalTrackFormat {
    pub grid_version: GridVersion,
    pub title: String,
    pub simulation_lines: Vec<SimulationLine>,
    pub scenery_lines: Vec<SceneryLine>,
    pub start_position: Vec2,
}
