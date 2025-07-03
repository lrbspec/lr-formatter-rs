use derive_builder::Builder;
use getset::{CloneGetters, CopyGetters};

use crate::track::{GridVersion, RGBColor, Vec2};

// TODO: replace bool props with feature set

#[derive(CopyGetters, CloneGetters, Debug, Builder)]
pub struct Metadata {
    // Shared Properties
    #[builder(default)]
    #[getset(get_copy = "pub")]
    grid_version: GridVersion,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    start_position: Option<Vec2>,

    // Linerider.com Properties
    #[builder(setter(strip_option, into), default)]
    #[getset(get_clone = "pub")]
    title: Option<String>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get_clone = "pub")]
    artist: Option<String>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get_clone = "pub")]
    description: Option<String>,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    duration: Option<u32>,
    #[builder(setter(strip_option, into), default)]
    #[getset(get_clone = "pub")]
    script: Option<String>,

    // LRA+ Properties
    #[builder(default)]
    #[getset(get_copy = "pub")]
    use_legacy_remount: bool,
    #[builder(default)]
    #[getset(get_copy = "pub")]
    use_legacy_fakie: bool,
    #[builder(default)]
    #[getset(get_copy = "pub")]
    zero_friction_riders: bool,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    gravity_well_size: Option<f64>,
    // TODO: Edit properties on rider group in solver
    #[builder(default)]
    #[getset(get_copy = "pub")]
    zero_start: bool,
    #[builder(default)]
    #[getset(get_copy = "pub")]
    remount: bool,
    #[builder(setter(strip_option, into), default)]
    #[getset(get_clone = "pub")]
    audio_filename: Option<String>,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    audio_offset_until_start: Option<f64>,

    // Flash Properties
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    start_line: Option<u32>,

    // Triggers
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    start_zoom: Option<f64>,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    start_gravity: Option<Vec2>,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    start_line_color: Option<RGBColor>,
    #[builder(setter(strip_option), default)]
    #[getset(get_copy = "pub")]
    start_background_color: Option<RGBColor>,
}
