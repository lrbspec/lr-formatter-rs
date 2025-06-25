use crate::internal::{rgb_color::RGBColor, trigger::FrameReachedEvent};
// TODO: Make this per layer

// TODO: This is intended for LRO color triggers acting on the base layer
#[derive(Debug, Clone)]
pub struct State {
    pub color: RGBColor,
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
