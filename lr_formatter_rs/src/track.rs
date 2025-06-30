mod feature_field_access;
mod grid_version;
mod properties;
mod rgb_color;
mod vec2;

use derive_more::Display;
use std::collections::HashSet;
use thiserror::Error;

use feature_field_access::{FeatureFieldAccess, UNREACHABLE_MESSAGE};
pub use grid_version::GridVersion;
pub use properties::{
    AccelerationLine, AccelerationLineBuilderError, Layer, LayerBuilderError, LayerFeature,
    LayerFolder, LayerFolderBuilderError, LayerGroup, LayerGroupBuilderError, LineFeature,
    LineGroup, LineGroupBuilderError, Metadata, MetadataBuilderError, Rider, RiderBuilderError,
    RiderFeature, RiderGroup, RiderGroupBuilderError, SceneryLine, SceneryLineBuilderError,
    StandardLine, StandardLineBuilderError,
};
pub use rgb_color::RGBColor;
pub use vec2::Vec2;

use crate::track::properties::{
    LayerGroupBuilder, LineGroupBuilder, MetadataBuilder, RiderGroupBuilder,
};

#[derive(Debug, Display, PartialEq, Eq, Hash)]
pub enum TrackFeature {
    Riders,
    Layers,
}

#[derive(Debug)]
pub struct Track {
    metadata: Metadata,
    line_group: LineGroup,
    layer_group: Option<LayerGroup>,
    rider_group: Option<RiderGroup>,
}

pub struct TrackBuilder {
    features: HashSet<TrackFeature>,
    line_group: LineGroupBuilder,
    metadata: MetadataBuilder,
    layer_group: Option<LayerGroupBuilder>,
    rider_group: Option<RiderGroupBuilder>,
}

impl Default for TrackBuilder {
    fn default() -> Self {
        Self {
            features: HashSet::new(),
            line_group: LineGroupBuilder::default(),
            metadata: MetadataBuilder::default(),
            layer_group: None,
            rider_group: None,
        }
    }
}

impl FeatureFieldAccess<TrackFeature, TrackBuilderError> for TrackBuilder {
    fn require_feature<'a, T>(
        &self,
        field: &'a Option<T>,
        feature: TrackFeature,
    ) -> Result<&'a T, TrackBuilderError> {
        if !self.features.contains(&feature) {
            return Err(TrackBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_ref() {
            Some(some_field) => Ok(some_field),
            None => unreachable!("{}", UNREACHABLE_MESSAGE),
        }
    }

    fn require_feature_mut<'a, T>(
        current_features: &HashSet<TrackFeature>,
        field: &'a mut Option<T>,
        feature: TrackFeature,
    ) -> Result<&'a mut T, TrackBuilderError> {
        if !current_features.contains(&feature) {
            return Err(TrackBuilderError::MissingFeatureFlag(feature));
        }

        match field.as_mut() {
            Some(some_field) => Ok(some_field),
            None => unreachable!("{}", UNREACHABLE_MESSAGE),
        }
    }

    fn check_feature<T>(
        &self,
        feature: TrackFeature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), TrackBuilderError> {
        if self.features.contains(&feature) && field.is_none() {
            return Err(TrackBuilderError::MissingAttribute(attr_name));
        }

        if !self.features.contains(&feature) && field.is_some() {
            return Err(TrackBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }
}

impl TrackBuilder {
    pub fn enable_feature(mut self, feature: TrackFeature) -> Self {
        if feature == TrackFeature::Layers && self.layer_group.is_none() {
            self.layer_group = Some(LayerGroupBuilder::default());
        }

        if feature == TrackFeature::Riders && self.rider_group.is_none() {
            self.rider_group = Some(RiderGroupBuilder::default());
        }

        self.features.insert(feature);
        self
    }

    pub fn get_metadata(&self) -> &MetadataBuilder {
        &self.metadata
    }

    pub fn get_line_group(&self) -> &LineGroupBuilder {
        &self.line_group
    }

    pub fn get_layer_group(&self) -> Result<&LayerGroupBuilder, TrackBuilderError> {
        Ok(self.require_feature(&self.layer_group, TrackFeature::Layers)?)
    }

    pub fn get_rider_group(&self) -> Result<&RiderGroupBuilder, TrackBuilderError> {
        Ok(self.require_feature(&self.rider_group, TrackFeature::Riders)?)
    }

    pub fn build(self) -> Result<Track, TrackBuilderError> {
        let metadata = self.metadata.build()?;
        let line_group = self.line_group.build()?;

        self.check_feature(TrackFeature::Layers, &self.layer_group, "layer_group");
        let layer_group = match self.layer_group.as_ref() {
            Some(layer_group_builder) => Some(layer_group_builder.build()?),
            None => None,
        };

        self.check_feature(TrackFeature::Layers, &self.rider_group, "rider_group");
        let rider_group = match self.rider_group.as_ref() {
            Some(rider_group_builder) => Some(rider_group_builder.build()?),
            None => None,
        };

        Ok(Track {
            metadata,
            line_group,
            layer_group,
            rider_group,
        })
    }
}

#[derive(Error, Debug)]
pub enum TrackBuilderError {
    #[error("Expected feature to be registered before passing feature data: {0}")]
    MissingFeatureFlag(TrackFeature),
    #[error("Expected feature data to be present because feature was enabled: {0}")]
    MissingAttribute(&'static str),
    #[error("{0}")]
    LineGroupBuilderError(#[from] LineGroupBuilderError),
    #[error("{0}")]
    LayerGroupBuilderError(#[from] LayerGroupBuilderError),
    #[error("{0}")]
    RiderGroupBuilderError(#[from] RiderGroupBuilderError),
    #[error("{0}")]
    MetadataBuilderError(#[from] MetadataBuilderError),
}
