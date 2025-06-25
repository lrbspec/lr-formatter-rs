pub mod background_color;
pub mod camera_focus;
pub mod camera_pan;
pub mod camera_zoom;
pub mod gravity;
pub mod layer_color;
pub mod layer_visibility;
pub mod timeline_speed;

/// A trigger event caused by the timeline reaching a specific frame
#[derive(Debug, Clone)]
pub struct FrameReachedEvent {
    pub frame: u32,
}

/// A trigger event caused by a line being hit
#[derive(Debug, Clone)]
pub struct LintHitEvent {
    pub line_id: u32,
}
