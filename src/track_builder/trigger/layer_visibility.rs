use crate::track_builder::trigger::FrameReachedEvent;
// TODO: Make this per layer

/// Determines the visibility of a layer by cycling it between on and off
#[derive(Debug, Clone)]
pub struct State {
    /// How long a layer is on during a cycle
    pub cycle_on: u32,
    /// How long a layer is off during a cycle
    pub cycle_off: u32,
    /// How many frames into the cycle this trigger starts (default is 0 frames into the "on" phase)
    pub offset: u32,
}

#[derive(Debug, Clone)]
pub struct Trigger {
    pub state: State,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct TriggerGroup {
    /// Whether to multiply by 1.5 to convert fps from forty to sixty
    pub sixty_fps_multiplier: bool,
    pub triggers: Vec<Trigger>,
}
