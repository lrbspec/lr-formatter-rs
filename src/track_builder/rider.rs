use crate::track_builder::vec2::Vec2;

#[derive(Debug, Clone)]
pub struct Rider {
    pub start_position: Vec2,
    pub start_velocity: Vec2,
    pub start_angle: f64,
    pub can_remount: bool,
}
