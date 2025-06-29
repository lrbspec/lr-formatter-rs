use derive_more::Display;
use getset::Getters;
use thiserror::Error;

use crate::track::properties::rider::rider_base::{Rider, RiderBuilder, RiderBuilderError};

#[derive(Debug, Display, PartialEq)]
pub enum RiderFeature {
    StartAngle,
    Remount,
}

#[derive(Debug, Getters)]
pub struct RiderGroup {
    #[getset(get = "pub")]
    riders: Vec<Rider>,
}

pub(in crate::track) struct RiderGroupBuilder {
    features: Vec<RiderFeature>,
    riders: Vec<RiderBuilder>,
}

impl RiderGroupBuilder {
    pub fn new() -> Self {
        Self {
            features: vec![],
            riders: vec![],
        }
    }

    pub fn enable_feature(mut self, feature: RiderFeature) -> Self {
        self.features.push(feature);
        self
    }

    fn check_feature<T>(
        &self,
        feature: RiderFeature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), RiderGroupBuilderError> {
        if self.features.contains(&feature) && field.is_none() {
            return Err(RiderGroupBuilderError::MissingAttribute(attr_name));
        }

        if !self.features.contains(&feature) && field.is_some() {
            return Err(RiderGroupBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }

    pub fn add_rider(mut self) -> Result<Self, RiderGroupBuilderError> {
        self.riders.push(RiderBuilder::default().to_owned());
        Ok(self)
    }

    // TODO rider modification methods

    pub fn build(&self) -> Result<RiderGroup, RiderGroupBuilderError> {
        let mut riders: Vec<Rider> = vec![];

        for rider_builder in &self.riders {
            let rider = rider_builder.build()?;
            self.check_feature(
                RiderFeature::StartAngle,
                &rider.start_angle(),
                "start_angle",
            );
            self.check_feature(RiderFeature::Remount, &rider.can_remount(), "can_remount");
            riders.push(rider);
        }

        Ok(RiderGroup { riders })
    }
}

#[derive(Error, Debug)]
pub enum RiderGroupBuilderError {
    #[error("Expected feature to be registered before passing feature data: {0}")]
    MissingFeatureFlag(RiderFeature),
    #[error("Expected feature data to be present because feature was enabled: {0}")]
    MissingAttribute(&'static str),
    #[error("{0}")]
    RiderBuilderError(#[from] RiderBuilderError),
}
