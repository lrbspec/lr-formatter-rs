use crate::track::{Vec2, props::trigger::FrameReachedEvent};
// TODO: Make this per rider

// TODO: LRO writes gravity as (0, 1) and scales internally, whereas .com writes gravity as (0, 0.175) and doesn't scale
#[derive(Debug, Clone)]
pub struct State {
    pub strength: Vec2,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub state: State,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct TriggerGroup {
    pub initial_state: State,
    pub triggers: Vec<Trigger>,
}
