use crate::track::Vec2;
use derive_builder::Builder;
use getset::Getters;

#[derive(Getters, Debug, Builder)]
#[getset(get = "pub")]
pub struct Rider {
    start_position: Vec2,
    start_velocity: Vec2,
    #[builder(setter(into, strip_option), default)]
    start_angle: Option<f64>,
    #[builder(setter(into, strip_option), default)]
    can_remount: Option<bool>,
}
