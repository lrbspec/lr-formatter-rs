//! Format that lr_formatter_rs parses into and out of, exposed for usage after reading and before writing
use strum::EnumDiscriminants;

#[derive(Debug, Clone)]
pub enum GridVersion {
    V6_2,
    V6_1,
    V6_0,
}

#[derive(Debug, Clone)]
pub struct RGBColor {
    red: u8,
    green: u8,
    blue: u8,
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

#[derive(Debug, Clone, EnumDiscriminants)]
pub enum LineTypeNew {
    Simulation {
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    },
    Acceleration {
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
        multiplier: f64,
    },
    Scenery {
        width: f64,
    },
}

#[derive(Debug, Clone)]
pub struct LineNew {
    pub id: u32,
    pub x1: f64,
    pub y1: f64,
    pub x2: f64,
    pub y2: f64,
    pub line_type: LineTypeNew,
}

/// Enum for describing track properties that can change by trigger events
#[derive(Debug, Clone)]
pub enum DynamicTrackProperty {
    CameraZoom {
        zoom_level: f64,
    },
    CameraPan {
        /// Percent camera size (based on zoom and viewport size) offset from default position
        percent_offset: Vec2,
        /// Width and height of the camera's collider, what the rider collides with to keep the camera focused
        bounds_collision_size: Vec2,
        /// Exact camera size in pixels offset from default position
        exact_offset: Vec2,
    },
    CameraFocus {
        /// Amount of weight each rider has on where the camera focuses
        rider_weights: Vec<f64>,
    },
    /// Known in linerider.com as "time remap" triggers, causes the timeline to speed up or slow down by some multiplier
    TimelineSpeed {
        speed_multiplier: f64,
    },
    Gravity(Vec2),
    /// Determines the visibility of a layer by cycling it between on and off
    LayerVisibility {
        /// How long a layer is on during a cycle
        cycle_on: u32,
        /// How long a layer is off during a cycle
        cycle_off: u32,
        /// How many frames into the cycle this trigger starts (default is 0 frames into the "on" phase)
        offset: u32,
    },
    LayerColor {
        layer_id: u32,
        color: RGBColor,
    },
    BackgroundColor {
        color: RGBColor,
    },
    SkinStyle {
        style_sheets: Vec<String>,
    },
}

/// Types of trigger events that can happen during a track
// TODO: Potentially move into trigger metadata property? (How?)
pub enum TriggerEventType {
    FrameReached { frame: u32 },
    LineHit { line_id: u32 },
}

// TODO: Finish this
#[derive(Debug, Clone)]
pub enum EventListMetadata {
    SmoothingStrength(f64),
    SmoothingEnabled(bool),
    SixtyFps(bool),
}

#[derive(Debug, Clone)]
pub struct Rider {
    start_position: Vec2,
    start_velocity: Vec2,
    start_angle: f64,
    can_remount: bool,
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
    file_name: String,
    // Offset (in seconds) to delay the start of the audio
    offset_until_start: f64,
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
    pub lines: Vec<LineNew>,
    pub audio: Option<Audio>,
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
            lines: vec![],
            audio: None,
        }
    }
}
