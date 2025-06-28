//! Utility for building tracks with extreme granularity

mod grid_version;
mod rgb_color;
mod vec2;

use attribute_flatten_struct::flatten_struct;
use derive_builder::Builder;
pub use grid_version::GridVersion;
pub use rgb_color::RGBColor;
pub use vec2::Vec2;

// The procedural macro `flatten_struct` is used here to flatten a bunch of structs into
// the parent struct, wrapped with Vec<>, then apply the derive_builder crate macros (which wraps
// the flattened props with Option<> for the builder pattern).

// This was done to self-document some of the natural groupings of certain objects (Riders, Layers,
// Lines, etc) without flattening them all into the builder struct directly, and keep consistency
// among prefixed file names so it's not as easy to make a mistake.

#[flatten_struct]
// These are injected by #[flatten_struct], because we need to do these *after* flattening:
//   #[derive(Builder)]
//   #[builder(private, build_fn(validate = "Self::validate"))]
#[derive(Debug)]
struct BTrack {
    // Shared Properties
    grid_version: GridVersion,
    #[builder(setter(strip_option), default)]
    start_position: Option<Vec2>,

    // Linerider.com Properties
    #[builder(setter(strip_option), default)]
    title: Option<String>,
    #[builder(setter(strip_option), default)]
    artist: Option<String>,
    #[builder(setter(strip_option), default)]
    description: Option<String>,
    #[builder(setter(strip_option), default)]
    duration: Option<u32>,
    #[builder(setter(strip_option), default)]
    script: Option<String>,

    // LRA+ Properties
    #[builder(default)]
    use_legacy_remount: bool,
    #[builder(default)]
    use_legacy_fakie: bool,
    #[builder(default)]
    zero_friction_riders: bool,
    #[builder(setter(strip_option), default)]
    gravity_well_size: Option<f64>,
    #[builder(default)]
    zero_start: bool,
    #[flatten(struct Audio2 {
      filename: String,
      offset_until_start: f64,
    })]
    audio: Audio2,
    initial_line_color: Option<RGBColor>,
    initial_background_color: Option<RGBColor>,
    initial_gravity_strength: Option<Vec2>,
    initial_zoom: Option<f64>,

    // Flash Properties
    #[builder(setter(strip_option), default)]
    start_line: Option<u32>,

    // Riders
    #[flatten(struct Rider2 {
        id: u32,
        position: Vec2,
        index: u32,
        velocity: Vec2,
        angle: f64,
        can_remount: bool
    })]
    rider: Vec<Rider2>,

    // Layers
    #[flatten(struct Layer2 {
      id: u32,
      index: u32,
      name: String,
      editable: bool,
      visible: bool,
      color: RGBColor,
      is_folder: bool,
      folder_size: Option<u32>,
      parent_folder: Option<u32>,
    })]
    layer: Vec<Layer2>,

    // Lines
    #[flatten(struct Line2 {
      id: u32,
      endpoints: (Vec2, Vec2),
      is_scenery: bool,
      scenery_line_width: f64,
      scenery_line_width_f32: f32,
      simulation_line_flags: u8,
      simulation_line_multiplier: f64
    })]
    line: Vec<Line2>,
    // TODO: Triggers
}

impl BTrackBuilder {
    // Validate that dependencies between props line up
    fn validate(&self) -> Result<(), String> {
        // TODO
        Ok(())
    }
}

// TODO: Remove everything below

#[derive(Debug, Clone)]
pub struct Audio {
    // File name of the audio relative to the directory the track file was located in during save
    pub file_name: String,
    // Offset (in seconds) to delay the start of the audio
    pub offset_until_start: f64,
}

use std::vec;

#[derive(Debug, Clone)]
pub struct SceneryLine {
    pub base_line: Line,
    pub width: Option<f64>,
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
pub struct Rider {
    pub start_position: Vec2,
    pub start_velocity: Vec2,
    pub start_angle: f64,
    pub can_remount: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineType {
    Standard = 0,
    Acceleration = 1,
    Scenery = 2,
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

/// Malleable struct for storing implementation-agnostic track properties
#[derive(Debug, Clone)]
pub struct Track {
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
    // pub camera_zoom_triggers: camera_zoom::TriggerGroup,
    // pub camera_pan_triggers: camera_pan::TriggerGroup,
    // pub camera_focus_triggers: camera_focus::TriggerGroup,
    // pub timeline_speed_triggers: timeline_speed::TriggerGroup,
    // pub gravity_triggers: gravity::TriggerGroup,
    // pub layer_visibility_triggers: layer_visibility::TriggerGroup,
    // pub layer_color_triggers: layer_color::TriggerGroup,
    // pub background_color_triggers: background_color::TriggerGroup,
}

impl Default for Track {
    fn default() -> Self {
        Self::new()
    }
}

impl Track {
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
        }
    }
}
