use crate::track::{
    Vec2,
    properties::line::{GetEndpoints, impl_get_endpoints},
};
use derive_builder::Builder;
use getset::Getters;

// TODO make mod for f32 vecs

#[derive(Getters, Debug, Builder)]
#[getset(get = "pub")]
pub struct SceneryLine {
    id: u32,
    #[getset(skip)]
    endpoints: (Vec2, Vec2),
    #[getset(skip)]
    #[builder(setter(into, strip_option), default)]
    width: Option<f64>,
    #[getset(skip)]
    #[builder(setter(into, strip_option), default)]
    width_f32: Option<f32>,
}

impl_get_endpoints! {SceneryLine}

pub enum SceneryWidth {
    F32(f32),
    F64(f64),
}

impl SceneryLine {
    pub fn width(&self) -> Option<SceneryWidth> {
        if let Some(inner_width) = self.width_f32 {
            Some(SceneryWidth::F32(inner_width))
        } else if let Some(inner_width) = self.width {
            Some(SceneryWidth::F64(inner_width))
        } else {
            None
        }
    }
}
