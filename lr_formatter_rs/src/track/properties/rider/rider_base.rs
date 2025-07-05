use crate::track::Vec2;
use derive_builder::Builder;
use getset::CopyGetters;

#[derive(CopyGetters, Debug, Builder)]
#[getset(get_copy = "pub")]
pub struct Rider {
    #[builder(default)]
    start_position: Option<Vec2>,
    #[builder(default)]
    start_velocity: Option<Vec2>,
    #[builder(default)]
    start_angle: Option<f64>,
    #[builder(default)]
    can_remount: Option<bool>,
}
