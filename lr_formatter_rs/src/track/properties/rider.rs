mod rider_base;
mod rider_group;

pub use rider_base::{Rider, RiderBuilderError};
pub(in crate::track) use rider_group::RiderGroupBuilder;
pub use rider_group::{RiderFeature, RiderGroup, RiderGroupBuilderError};
