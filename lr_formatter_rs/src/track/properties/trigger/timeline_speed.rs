use crate::track::properties::trigger::FrameReachedEvent;

/// Known in linerider.com as "time remap" triggers, causes the timeline to speed up or slow down by some multiplier
#[derive(Debug, Clone)]
pub struct State {
    pub speed_multiplier: f64,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub state: State,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct TriggerGroup {
    pub initial_state: State,
    pub interpolate_speeds: bool,
    pub triggers: Vec<Trigger>,
}
