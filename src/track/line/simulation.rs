use crate::track::Line;

#[derive(Debug, Clone)]
pub struct SimulationLine {
    pub base_line: Line,
    pub flipped: bool,
    pub left_extension: bool,
    pub right_extension: bool,
    pub multiplier: Option<f64>,
}
