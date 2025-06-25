use crate::internal::{trigger::FrameReachedEvent, vec2::Vec2};

#[derive(Debug, Clone)]
pub struct State {
    /// Percent camera size (based on zoom and viewport size) offset from default position
    pub percent_offset: Vec2,
    /// Width and height of the camera's collider, what the rider collides with to keep the camera focused
    pub bounds_collision_size: Vec2,
    /// Exact camera size in pixels offset from default position
    pub exact_offset: Vec2,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub state: State,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct TriggerGroup {
    pub initial_state: State,
    pub smoothing: u32,
    pub triggers: Vec<Trigger>,
}
