use crate::track::Line;

#[derive(Debug, Clone)]
pub struct SceneryLine {
    pub base_line: Line,
    pub width: Option<f64>,
}
