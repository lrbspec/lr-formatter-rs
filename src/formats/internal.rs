//! Format that lr_formatter_rs parses into and out of, exposed for usage after reading and before writing

mod audio;
mod grid_version;
mod layer;
mod rgb_color;
mod rider;
mod track_line;
mod trigger;
mod vec2;

use std::vec;

pub use audio::Audio;
pub use grid_version::GridVersion;
pub use layer::Layer;
pub use rgb_color::RGBColor;
pub use rider::Rider;
pub use track_line::{Line, LineType, SceneryLine, SimulationLine};
pub use vec2::Vec2;

use crate::internal::trigger::{
    background_color, camera_focus, camera_pan, camera_zoom, gravity, layer_color,
    layer_visibility, timeline_speed,
};

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
    pub camera_zoom_triggers: camera_zoom::TriggerGroup,
    pub camera_pan_triggers: camera_pan::TriggerGroup,
    pub camera_focus_triggers: camera_focus::TriggerGroup,
    pub timeline_speed_triggers: timeline_speed::TriggerGroup,
    pub gravity_triggers: gravity::TriggerGroup,
    pub layer_visibility_triggers: layer_visibility::TriggerGroup,
    pub layer_color_triggers: layer_color::TriggerGroup,
    pub background_color_triggers: background_color::TriggerGroup,
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
            camera_zoom_triggers: camera_zoom::TriggerGroup {
                smoothing: 20,
                triggers: vec![],
                legacy_triggers: vec![],
                initial_state: camera_zoom::State { zoom_level: 1.0 },
            },
            camera_pan_triggers: camera_pan::TriggerGroup {
                smoothing: 20,
                triggers: vec![],
                initial_state: camera_pan::State {
                    exact_offset: Vec2 { x: 0.0, y: 0.0 },
                    bounds_collision_size: Vec2 { x: 0.4, y: 0.4 },
                    percent_offset: Vec2 { x: 0.0, y: 0.0 },
                },
            },
            camera_focus_triggers: camera_focus::TriggerGroup {
                smoothing: 20,
                triggers: vec![],
                initial_state: camera_focus::State {
                    rider_weights: vec![1.0, 0.0, 0.0],
                },
            },
            timeline_speed_triggers: timeline_speed::TriggerGroup {
                interpolate_speeds: false,
                triggers: vec![],
                initial_state: timeline_speed::State {
                    speed_multiplier: 1.0,
                },
            },
            gravity_triggers: gravity::TriggerGroup {
                triggers: vec![],
                initial_state: gravity::State {
                    strength: Vec2 { x: 0.0, y: 1.0 },
                },
            },
            layer_visibility_triggers: layer_visibility::TriggerGroup {
                sixty_fps_multiplier: false,
                triggers: vec![],
            },
            layer_color_triggers: layer_color::TriggerGroup {
                triggers: vec![],
                initial_state: layer_color::State {
                    color: RGBColor {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                },
            },
            background_color_triggers: background_color::TriggerGroup {
                triggers: vec![],
                initial_state: background_color::State {
                    color: RGBColor {
                        red: 255,
                        green: 255,
                        blue: 255,
                    },
                },
            },
        }
    }
}
