use crate::track::props::trigger::FrameReachedEvent;

#[derive(Debug, Clone)]
pub struct State {
    /// Amount of weight each rider has on where the camera focuses
    pub rider_weights: Vec<f64>,
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
