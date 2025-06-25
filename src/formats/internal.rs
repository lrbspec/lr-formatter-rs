//! Format that lr_formatter_rs parses into and out of, exposed for usage after reading and before writing

#[derive(Debug, Clone)]
pub enum GridVersion {
    V6_2,
    V6_1,
    V6_0,
}

#[derive(Debug, Clone)]
pub struct RGBColor {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    BLUE = 0,
    RED = 1,
    GREEN = 2,
}

#[derive(Debug, Clone)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone)]
pub struct Line {
    pub id: u32,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub line_type: LineType,
}

#[derive(Debug, Clone)]
pub struct SimulationLine {
    pub base_line: Line,
    pub flipped: bool,
    pub left_extension: bool,
    pub right_extension: bool,
    pub multiplier: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct SceneryLine {
    pub base_line: Line,
    pub width: Option<f64>,
}

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

// TODO: Figure out how to convert between LRO zoom and .com zoom
#[derive(Debug, Clone)]
pub struct CameraZoomTrigger {
    pub zoom_level: f64,
    pub trigger_event: FrameReachedEvent,
}

// TODO: Figure out how to convert this to a regular camera zoom trigger
#[derive(Debug, Clone)]
pub struct LegacyCameraZoomTrigger {
    pub zoom_level: f64,
    pub trigger_event: LintHitEvent,
}

#[derive(Debug, Clone)]
pub struct CameraPanTrigger {
    /// Percent camera size (based on zoom and viewport size) offset from default position
    pub percent_offset: Vec2,
    /// Width and height of the camera's collider, what the rider collides with to keep the camera focused
    pub bounds_collision_size: Vec2,
    /// Exact camera size in pixels offset from default position
    pub exact_offset: Vec2,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct CameraFocusTrigger {
    /// Amount of weight each rider has on where the camera focuses
    pub rider_weights: Vec<f64>,
    pub trigger_event: FrameReachedEvent,
}

/// Known in linerider.com as "time remap" triggers, causes the timeline to speed up or slow down by some multiplier
#[derive(Debug, Clone)]
pub struct TimelineSpeedTrigger {
    pub speed_multiplier: f64,
    pub trigger_event: FrameReachedEvent,
}

// TODO: LRO writes gravity as (0, 1) and scales internally, whereas .com writes gravity as (0, 0.175) and doesn't scale
#[derive(Debug, Clone)]
pub struct GravityTrigger {
    pub strength: Vec2,
    pub trigger_event: FrameReachedEvent,
}

/// Determines the visibility of a layer by cycling it between on and off
#[derive(Debug, Clone)]
pub struct LayerVisibilityTrigger {
    /// How long a layer is on during a cycle
    pub cycle_on: u32,
    /// How long a layer is off during a cycle
    pub cycle_off: u32,
    /// How many frames into the cycle this trigger starts (default is 0 frames into the "on" phase)
    pub offset: u32,
    pub trigger_event: FrameReachedEvent,
}

// TODO: This is intended for LRO color triggers acting on the base layer
#[derive(Debug, Clone)]
pub struct LayerColorTrigger {
    pub layer_id: u32,
    pub color: RGBColor,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct BackgroundColorTrigger {
    pub color: RGBColor,
    pub trigger_event: FrameReachedEvent,
}

#[derive(Debug, Clone)]
pub struct CameraZoomTriggerGroup {
    pub smoothing: u32,
    pub triggers: Vec<CameraZoomTrigger>,
}

#[derive(Debug, Clone)]
pub struct CameraPanTriggerGroup {
    pub smoothing: u32,
    pub triggers: Vec<CameraPanTrigger>,
}

#[derive(Debug, Clone)]
pub struct CameraFocusTriggerGroup {
    pub smoothing: u32,
    pub triggers: Vec<CameraFocusTrigger>,
}

#[derive(Debug, Clone)]
pub struct TimelineSpeedTriggerGroup {
    pub interpolate_speeds: bool,
    pub triggers: Vec<TimelineSpeedTrigger>,
}

#[derive(Debug, Clone)]
pub struct LayerVisibilityTriggerGroup {
    pub sixty_fps: bool,
    pub triggers: Vec<LayerVisibilityTrigger>,
}

#[derive(Debug, Clone)]
pub struct Rider {
    pub start_position: Vec2,
    pub start_velocity: Vec2,
    pub start_angle: f64,
    pub can_remount: bool,
}

#[derive(Debug, Clone)]
pub enum Layer {
    Layer {
        id: u32,
        name: String,
        color: RGBColor,
        visible: bool,
        editable: bool,
        folder_id: Option<u32>,
    },
    Folder {
        id: u32,
        name: String,
        visible: bool,
        editable: bool,
        size: u32,
    },
}

#[derive(Debug, Clone)]
pub struct Audio {
    // File name of the audio relative to the directory the track file was located in during save
    pub file_name: String,
    // Offset (in seconds) to delay the start of the audio
    pub offset_until_start: f64,
}

/// Malleable struct for storing implementation-agnostic track properties
#[derive(Debug, Clone)]
pub struct InternalTrackFormat {
    pub grid_version: GridVersion,
    pub title: String,
    pub artist: String,
    pub description: String,
    pub duration: u32,
    pub script: String,
    pub simulation_lines: Vec<SimulationLine>,
    pub scenery_lines: Vec<SceneryLine>,
    pub start_position: Vec2,
    pub use_legacy_remount: bool,
    pub zero_friction_riders: bool,
    pub gravity_well_size: f64,
    pub riders: Vec<Rider>,
    pub layers: Vec<Layer>,
    pub audio: Option<Audio>,
    pub rider_skin_stylesheets: Vec<String>,
    pub camera_zoom_triggers: CameraZoomTriggerGroup,
    pub legacy_camera_zoom_triggers: Vec<LegacyCameraZoomTrigger>,
    pub camera_pan_triggers: CameraPanTriggerGroup,
    pub camera_focus_triggers: CameraFocusTriggerGroup,
    pub timeline_speed_triggers: TimelineSpeedTriggerGroup,
    pub gravity_triggers: Vec<GravityTrigger>,
    pub layer_visibility_triggers: LayerVisibilityTriggerGroup,
    pub layer_color_triggers: Vec<LayerColorTrigger>,
    pub background_color_triggers: Vec<BackgroundColorTrigger>,
}

impl InternalTrackFormat {
    /// Creates a new `InternalTrackFormat` with recommended defaults
    pub fn new() -> Self {
        Self {
            scenery_lines: vec![],
            simulation_lines: vec![],
            grid_version: GridVersion::V6_2,
            use_legacy_remount: false,
            zero_friction_riders: false,
            gravity_well_size: 10.0,
            title: String::new(),
            artist: String::new(),
            description: String::new(),
            duration: 1200,
            script: String::new(),
            start_position: Vec2 { x: 0.0, y: 0.0 },
            riders: vec![Rider {
                start_position: Vec2 { x: 0.0, y: 0.0 },
                start_velocity: Vec2 { x: 0.4, y: 0.0 },
                start_angle: 0.0,
                can_remount: true,
            }],
            layers: vec![Layer::Layer {
                id: 0,
                name: "Base Layer".to_string(),
                color: RGBColor {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                visible: true,
                editable: true,
                folder_id: None,
            }],
            audio: None,
            rider_skin_stylesheets: vec![],
            camera_zoom_triggers: CameraZoomTriggerGroup {
                smoothing: 20,
                triggers: vec![CameraZoomTrigger {
                    zoom_level: 1.0,
                    trigger_event: FrameReachedEvent { frame: 0 },
                }],
            },
            legacy_camera_zoom_triggers: vec![],
            camera_pan_triggers: CameraPanTriggerGroup {
                smoothing: 20,
                triggers: vec![CameraPanTrigger {
                    exact_offset: Vec2 { x: 0.0, y: 0.0 },
                    bounds_collision_size: Vec2 { x: 0.4, y: 0.4 },
                    percent_offset: Vec2 { x: 0.0, y: 0.0 },
                    trigger_event: FrameReachedEvent { frame: 0 },
                }],
            },
            camera_focus_triggers: CameraFocusTriggerGroup {
                smoothing: 20,
                triggers: vec![CameraFocusTrigger {
                    rider_weights: vec![1.0, 0.0, 0.0],
                    trigger_event: FrameReachedEvent { frame: 0 },
                }],
            },
            timeline_speed_triggers: TimelineSpeedTriggerGroup {
                interpolate_speeds: false,
                triggers: vec![TimelineSpeedTrigger {
                    speed_multiplier: 1.0,
                    trigger_event: FrameReachedEvent { frame: 0 },
                }],
            },
            gravity_triggers: vec![GravityTrigger {
                strength: Vec2 { x: 0.0, y: 1.0 },
                trigger_event: FrameReachedEvent { frame: 0 },
            }],
            layer_visibility_triggers: LayerVisibilityTriggerGroup {
                sixty_fps: false,
                triggers: vec![],
            },
            layer_color_triggers: vec![LayerColorTrigger {
                layer_id: 0,
                color: RGBColor {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                trigger_event: FrameReachedEvent { frame: 0 },
            }],
            background_color_triggers: vec![BackgroundColorTrigger {
                color: RGBColor {
                    red: 255,
                    green: 255,
                    blue: 255,
                },
                trigger_event: FrameReachedEvent { frame: 0 },
            }],
        }
    }
}
