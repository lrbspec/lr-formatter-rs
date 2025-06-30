mod acceleration_line;
mod line_group;
mod scenery_line;
mod standard_line;

pub use acceleration_line::{
    AccelerationLine, AccelerationLineBuilder, AccelerationLineBuilderError,
};
pub use line_group::{LineFeature, LineGroup, LineGroupBuilder, LineGroupBuilderError};
pub use scenery_line::{SceneryLine, SceneryLineBuilder, SceneryLineBuilderError};
pub use standard_line::{StandardLine, StandardLineBuilder, StandardLineBuilderError};

pub trait GetEndpoints {
    fn x1(&self) -> f64;
    fn y1(&self) -> f64;
    fn x2(&self) -> f64;
    fn y2(&self) -> f64;
}

macro_rules! impl_get_endpoints {
    ($t:ty) => {
        impl GetEndpoints for $t {
            fn x1(&self) -> f64 {
                self.endpoints.0.x
            }

            fn y1(&self) -> f64 {
                self.endpoints.0.y
            }

            fn x2(&self) -> f64 {
                self.endpoints.1.x
            }

            fn y2(&self) -> f64 {
                self.endpoints.1.y
            }
        }
    };
}

pub(in crate::track::properties::line) use impl_get_endpoints;
