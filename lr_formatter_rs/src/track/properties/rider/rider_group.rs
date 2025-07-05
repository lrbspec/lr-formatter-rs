use std::collections::HashSet;

use derive_more::Display;
use getset::Getters;
use thiserror::Error;

use crate::track::{
    FeatureFieldAccess, UNREACHABLE_MESSAGE,
    properties::rider::rider_base::{Rider, RiderBuilder, RiderBuilderError},
};

#[derive(Debug, Display, PartialEq, Eq, Hash)]
pub enum RiderFeature {
    StartAngle,
    Remount,
}

#[derive(Debug, Getters)]
#[getset(get = "pub")]
pub struct RiderGroup {
    riders: Vec<Rider>,
}

pub struct RiderGroupBuilder {
    features: HashSet<RiderFeature>,
    riders: Vec<RiderBuilder>,
}

impl Default for RiderGroupBuilder {
    fn default() -> Self {
        Self {
            features: HashSet::new(),
            riders: vec![],
        }
    }
}

impl FeatureFieldAccess<RiderFeature, RiderGroupBuilderError> for RiderGroupBuilder {
    fn require_feature<'a, T>(
        current_features: &HashSet<RiderFeature>,
        field: &'a mut Option<T>,
        feature: RiderFeature,
    ) -> Result<&'a mut T, RiderGroupBuilderError> {
        if !current_features.contains(&feature) {
            return Err(RiderGroupBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_mut() {
            Some(some_field) => Ok(some_field),
            None => unreachable!("{}", UNREACHABLE_MESSAGE),
        }
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
}

impl RiderGroupBuilder {
    pub fn enable_feature(&mut self, feature: RiderFeature) -> &mut Self {
        self.features.insert(feature);
        self
    }

    pub fn add_rider(&mut self) -> &mut RiderBuilder {
        self.riders.push(RiderBuilder::default().to_owned());
        self.riders.last_mut().unwrap()
    }

    pub fn get_riders(&mut self) -> impl Iterator<Item = &mut RiderBuilder> {
        self.riders.iter_mut()
    }

    pub fn build(&mut self) -> Result<RiderGroup, RiderGroupBuilderError> {
        let mut riders: Vec<Rider> = vec![];

        for rider_builder in &self.riders {
            let rider = rider_builder.build()?;
            self.check_feature(
                RiderFeature::StartAngle,
                &rider.start_angle(),
                "start_angle",
            )?;
            self.check_feature(RiderFeature::Remount, &rider.can_remount(), "can_remount")?;
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
