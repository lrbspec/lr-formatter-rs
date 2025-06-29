use derive_builder::Builder;
use getset::Getters;

use crate::track::{GridVersion, Vec2};

#[derive(Getters, Debug, Builder)]
pub struct Metadata {
    // Shared Properties
    #[getset(get = "pub")]
    grid_version: GridVersion,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    start_position: Option<Vec2>,

    // Linerider.com Properties
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    title: Option<String>,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    artist: Option<String>,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    description: Option<String>,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    duration: Option<u32>,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    script: Option<String>,

    // LRA+ Properties
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    use_legacy_remount: bool,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    use_legacy_fakie: bool,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    zero_friction_riders: bool,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    gravity_well_size: Option<f64>,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    zero_start: bool,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    audio_filename: Option<String>,
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    audio_offset_until_start: Option<f64>,

    // Flash Properties
    #[getset(get = "pub")]
    #[builder(setter(into, strip_option), default)]
    start_line: Option<u32>,
}
