pub mod lrajson;
pub mod lrb;
pub mod lrpk;
pub mod trackjson;
pub mod trk;

pub enum Format {
    TrackJson,
    LRB,
    LRAJson,
    TRK,
    LRPK,
}

#[derive(Debug, Clone, Default)]
enum GridVersion {
    #[default]
    V6_2 = 0,
    V6_1 = 1,
    V6_0 = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LineType {
    BLUE = 0,
    RED = 1,
    GREEN = 2,
}

#[derive(Debug, Clone, Default)]
struct Vec2 {
    x: f64,
    y: f64,
}

#[derive(Debug, Clone)]
struct Line {
    id: u32,
    x1: f64,
    y1: f64,
    x2: f64,
    y2: f64,
    line_type: LineType,
}

#[derive(Debug, Clone)]
struct SimulationLine {
    base_line: Line,
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    multiplier: Option<f64>,
}

#[derive(Debug, Clone)]
struct SceneryLine {
    base_line: Line,
    width: Option<f64>,
}

#[derive(Debug, Clone, Default)]
pub struct InternalTrackFormat {
    grid_version: GridVersion,
    title: String,
    artist: String,
    description: String,
    duration: u32,
    script: String,
    simulation_lines: Vec<SimulationLine>,
    scenery_lines: Vec<SceneryLine>,
    start_position: Vec2,
}

impl InternalTrackFormat {
    fn filled_default() -> Self {
        Self {
            duration: 1200,
            ..Default::default()
        }
    }
}
