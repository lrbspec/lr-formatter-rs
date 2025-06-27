mod scenery;
mod simulation;

pub use scenery::SceneryLine;
pub use simulation::SimulationLine;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    Standard = 0,
    Acceleration = 1,
    Scenery = 2,
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
