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
pub enum TrackFeature {}

#[derive(Debug)]
pub struct Track {
    line_group: LineGroup,
    layer_group: Option<LayerGroup>,
    rider_group: Option<RiderGroup>,
    metadata: Option<Metadata>,
}

pub struct TrackBuilder {
    features: Vec<TrackFeature>,
    line_group: LineGroupBuilder,
    layer_group: Option<LayerGroupBuilder>,
    rider_group: Option<RiderGroupBuilder>,
    metadata: Option<MetadataBuilder>,
}

impl TrackBuilder {
    pub fn new() -> Self {
        Self {
            features: vec![],
            line_group: LineGroupBuilder::new(),
            layer_group: None,
            rider_group: None,
            metadata: None,
        }
    }

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
        let metadata = match self.metadata.as_ref() {
            Some(metadata_builder) => Some(metadata_builder.build()?),
            None => None,
        };

        let rider_group = match self.rider_group.as_ref() {
            Some(rider_group_builder) => Some(rider_group_builder.build()?),
            None => None,
        };

        let layer_group = match self.layer_group.as_ref() {
            Some(layer_group_builder) => Some(layer_group_builder.build()?),
            None => None,
        };

        let line_group = self.line_group.build()?;

        Ok(Track {
            metadata,
            layer_group,
            line_group,
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
