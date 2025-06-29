use crate::track::{
    Vec2,
    properties::line::{GetEndpoints, impl_get_endpoints},
};
use derive_builder::Builder;
use getset::Getters;

#[derive(Getters, Debug, Builder)]
#[getset(get = "pub")]
pub struct AccelerationLine {
    id: u32,
    #[getset(skip)]
    endpoints: (Vec2, Vec2),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
    #[builder(setter(into, strip_option), default)]
    multiplier: Option<f64>,
}

impl_get_endpoints! {AccelerationLine}
