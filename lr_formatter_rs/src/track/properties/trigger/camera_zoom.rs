use crate::track::properties::trigger::{FrameReachedEvent, LintHitEvent};

#[derive(Debug, Clone)]
pub struct State {
    pub zoom_level: f64,
}

// TODO: Figure out how to convert between LRO zoom and .com zoom
#[derive(Debug, Clone)]
pub struct Trigger {
    pub state: State,
    pub trigger_event: FrameReachedEvent,
}

// TODO: Figure out how to convert this to a regular camera zoom trigger
#[derive(Debug, Clone)]
pub struct LegacyTrigger {
    pub state: State,
    pub trigger_event: LintHitEvent,
}

#[derive(Debug, Clone)]
pub struct TriggerGroup {
    pub smoothing: u32,
    pub triggers: Vec<Trigger>,
    pub legacy_triggers: Vec<LegacyTrigger>,
    pub initial_state: State,
}
