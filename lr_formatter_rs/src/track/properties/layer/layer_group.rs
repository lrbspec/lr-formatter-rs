use derive_more::Display;
use getset::Getters;
use thiserror::Error;

use crate::track::properties::layer::{
    layer_base::{Layer, LayerBuilder, LayerBuilderError},
    layer_folder::{LayerFolder, LayerFolderBuilder, LayerFolderBuilderError},
};

#[derive(Debug, Display, PartialEq)]
pub enum LayerFeature {
    Name,
    Visible,
    Editable,
    Folders,
}

#[derive(Debug, Getters)]
pub struct LayerGroup {
    #[getset(get = "pub")]
    layers: Vec<Layer>,
    #[getset(get = "pub")]
    layer_folders: Option<Vec<LayerFolder>>,
}

pub(in crate::track) struct LayerGroupBuilder {
    features: Vec<LayerFeature>,
    layers: Vec<LayerBuilder>,
    layer_folders: Option<Vec<LayerFolderBuilder>>,
}

impl LayerGroupBuilder {
    pub fn new() -> Self {
        Self {
            features: vec![],
            layers: vec![],
            layer_folders: None,
        }
    }

    pub fn enable_feature(mut self, feature: LayerFeature) -> Self {
        if feature == LayerFeature::Folders {
            self.layer_folders = Some(vec![]);
        }

        self.features.push(feature);
        self
    }

    fn check_feature<T>(
        &self,
        feature: LayerFeature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), LayerGroupBuilderError> {
        if self.features.contains(&feature) && field.is_none() {
            return Err(LayerGroupBuilderError::MissingAttribute(attr_name));
        }

        if !self.features.contains(&feature) && field.is_some() {
            return Err(LayerGroupBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }

    pub fn add_layer(mut self, id: u32, index: usize) -> Result<Self, LayerGroupBuilderError> {
        self.layers
            .push(LayerBuilder::default().id(id).index(index).to_owned());

        Ok(self)
    }

    pub fn add_layer_folder(
        mut self,
        id: u32,
        index: usize,
    ) -> Result<Self, LayerGroupBuilderError> {
        if !self.features.contains(&LayerFeature::Folders) {
            return Err(LayerGroupBuilderError::MissingFeatureFlag(
                LayerFeature::Folders,
            ));
        }

        match self.layer_folders.as_mut() {
            Some(layer_folders) => {
                layer_folders.push(LayerFolderBuilder::default().id(id).index(index).to_owned());
            }
            None => unreachable!(
                "BUG: Layer folder list should have been initialized when including feature"
            ),
        }

        Ok(self)
    }

    // TODO layer modification methods

    pub fn build(&self) -> Result<LayerGroup, LayerGroupBuilderError> {
        let mut layers: Vec<Layer> = vec![];
        let mut layer_folders: Option<Vec<LayerFolder>> = None;

        for layer_builder in &self.layers {
            let layer = layer_builder.build()?;
            self.check_feature(LayerFeature::Name, &layer.name(), "name");
            self.check_feature(LayerFeature::Visible, &layer.visible(), "visible");
            self.check_feature(LayerFeature::Editable, &layer.editable(), "editable");
            self.check_feature(LayerFeature::Folders, &layer.folder_id(), "folder_id");
            layers.push(layer);
        }

        self.check_feature(LayerFeature::Folders, &self.layer_folders, "layer_folders");
        if let Some(layer_folder_builders) = &self.layer_folders {
            let mut some_layer_folders = vec![];
            for layer_folder_builder in layer_folder_builders {
                let layer_folder = layer_folder_builder.build()?;
                self.check_feature(LayerFeature::Name, &layer_folder.name(), "name");
                self.check_feature(LayerFeature::Visible, &layer_folder.visible(), "visible");
                self.check_feature(LayerFeature::Editable, &layer_folder.editable(), "editable");
                some_layer_folders.push(layer_folder);
            }
            layer_folders = Some(some_layer_folders);
        }

        Ok(LayerGroup {
            layers,
            layer_folders,
        })
    }
}

#[derive(Error, Debug)]
pub enum LayerGroupBuilderError {
    #[error("Expected feature to be registered before passing feature data: {0}")]
    MissingFeatureFlag(LayerFeature),
    #[error("Expected feature data to be present because feature was enabled: {0}")]
    MissingAttribute(&'static str),
    #[error("{0}")]
    LayerBuilderError(#[from] LayerBuilderError),
    #[error("{0}")]
    LayerFolderBuilderError(#[from] LayerFolderBuilderError),
}
