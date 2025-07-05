use crate::track::Vec2;
use derive_builder::Builder;
use getset::CopyGetters;

// TODO make mod for f32 vecs

#[derive(CopyGetters, Debug, Builder)]
#[getset(get_copy = "pub")]
pub struct SceneryLine {
    id: u32,
    #[getset(skip)]
    endpoints: (Vec2, Vec2),
    #[builder(default)]
    width: Option<f64>,
}

impl SceneryLine {
    pub fn x1(&self) -> f64 {
        self.endpoints.0.x
    }

    pub fn y1(&self) -> f64 {
        self.endpoints.0.y
    }

    pub fn x2(&self) -> f64 {
        self.endpoints.1.x
    }

    pub fn y2(&self) -> f64 {
        self.endpoints.1.y
    }
}
