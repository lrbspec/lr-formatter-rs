use crate::track::{
    Vec2,
    properties::line::{GetEndpoints, impl_get_endpoints},
};
use derive_builder::Builder;
use getset::Getters;

#[derive(Getters, Debug, Builder)]
#[getset(get = "pub")]
pub struct StandardLine {
    id: u32,
    #[getset(skip)]
    endpoints: (Vec2, Vec2),
    flipped: bool,
    left_extension: bool,
    right_extension: bool,
}

impl_get_endpoints! {StandardLine}
