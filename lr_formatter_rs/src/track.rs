mod grid_version;
mod properties;
mod rgb_color;
mod vec2;

use derive_more::Display;
use thiserror::Error;

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

#[derive(Debug, Display, PartialEq)]
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
    features: Vec<TrackFeature>,
    line_group: LineGroupBuilder,
    metadata: MetadataBuilder,
    layer_group: Option<LayerGroupBuilder>,
    rider_group: Option<RiderGroupBuilder>,
}

impl Default for TrackBuilder {
    fn default() -> Self {
        Self {
            features: vec![],
            line_group: LineGroupBuilder::default(),
            metadata: MetadataBuilder::default(),
            layer_group: None,
            rider_group: None,
        }
    }
}

impl TrackBuilder {
    pub fn enable_feature(mut self, feature: TrackFeature) -> Self {
        self.features.push(feature);
        self
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

    // TODO methods

    pub fn build(self) -> Result<Track, TrackBuilderError> {
        let metadata = self.metadata.build()?;
        let line_group = self.line_group.build()?;

        let layer_group = match self.layer_group.as_ref() {
            Some(layer_group_builder) => Some(layer_group_builder.build()?),
            None => None,
        };

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
