use derive_more::Display;
use getset::Getters;
use thiserror::Error;

use crate::track::{
    properties::line::{
        acceleration_line::{
            AccelerationLine, AccelerationLineBuilder, AccelerationLineBuilderError,
        },
        scenery_line::{SceneryLine, SceneryLineBuilder, SceneryLineBuilderError},
        standard_line::{StandardLine, StandardLineBuilder, StandardLineBuilderError},
    },
    vec2::Vec2,
};

#[derive(Debug, Display, PartialEq)]
pub enum LineFeature {
    SceneryWidth,
    AccelerationMultiplier,
    SinglePrecisionSceneryWidth,
}

#[derive(Debug, Getters)]
pub struct LineGroup {
    #[getset(get = "pub")]
    standard_lines: Vec<StandardLine>,
    #[getset(get = "pub")]
    acceleration_lines: Vec<AccelerationLine>,
    #[getset(get = "pub")]
    scenery_lines: Vec<SceneryLine>,
}

pub(in crate::track) struct LineGroupBuilder {
    features: Vec<LineFeature>,
    standard_lines: Vec<StandardLineBuilder>,
    acceleration_lines: Vec<AccelerationLineBuilder>,
    scenery_lines: Vec<SceneryLineBuilder>,
}

impl LineGroupBuilder {
    pub fn new() -> Self {
        Self {
            features: vec![],
            standard_lines: vec![],
            acceleration_lines: vec![],
            scenery_lines: vec![],
        }
    }

    pub fn enable_feature(mut self, feature: LineFeature) -> Self {
        self.features.push(feature);
        self
    }

    fn check_feature<T>(
        &self,
        feature: LineFeature,
        field: &Option<T>,
        attr_name: &'static str,
    ) -> Result<(), LineGroupBuilderError> {
        if self.features.contains(&feature) && field.is_none() {
            return Err(LineGroupBuilderError::MissingAttribute(attr_name));
        }

        if !self.features.contains(&feature) && field.is_some() {
            return Err(LineGroupBuilderError::MissingFeatureFlag(feature));
        }

        Ok(())
    }

    pub fn add_standard_line(
        mut self,
        id: u32,
        end_points: (Vec2, Vec2),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> Result<Self, LineGroupBuilderError> {
        self.standard_lines.push(
            StandardLineBuilder::default()
                .id(id)
                .endpoints(end_points)
                .flipped(flipped)
                .left_extension(left_extension)
                .right_extension(right_extension)
                .to_owned(),
        );

        Ok(self)
    }

    pub fn add_acceleration_line(
        mut self,
        id: u32,
        end_points: (Vec2, Vec2),
        flipped: bool,
        left_extension: bool,
        right_extension: bool,
    ) -> Result<Self, LineGroupBuilderError> {
        self.acceleration_lines.push(
            AccelerationLineBuilder::default()
                .id(id)
                .endpoints(end_points)
                .flipped(flipped)
                .left_extension(left_extension)
                .right_extension(right_extension)
                .to_owned(),
        );

        Ok(self)
    }

    pub fn add_scenery_line(
        mut self,
        id: u32,
        end_points: (Vec2, Vec2),
    ) -> Result<Self, LineGroupBuilderError> {
        self.scenery_lines.push(
            SceneryLineBuilder::default()
                .id(id)
                .endpoints(end_points)
                .to_owned(),
        );

        Ok(self)
    }

    // TODO line modification methods

    pub fn build(&self) -> Result<LineGroup, LineGroupBuilderError> {
        let mut standard_lines: Vec<StandardLine> = vec![];
        let mut acceleration_lines: Vec<AccelerationLine> = vec![];
        let mut scenery_lines: Vec<SceneryLine> = vec![];

        for standard_line_builder in &self.standard_lines {
            let standard_line = standard_line_builder.build()?;
            standard_lines.push(standard_line);
        }

        for acceleration_line_builder in &self.acceleration_lines {
            let acceleration_line = acceleration_line_builder.build()?;
            self.check_feature(
                LineFeature::AccelerationMultiplier,
                &acceleration_line.multiplier(),
                "multiplier",
            )?;
            acceleration_lines.push(acceleration_line);
        }

        for scenery_line_builder in &self.scenery_lines {
            let scenery_line = scenery_line_builder.build()?;
            self.check_feature(LineFeature::SceneryWidth, &scenery_line.width(), "width")?;
            scenery_lines.push(scenery_line);
        }

        Ok(LineGroup {
            standard_lines,
            acceleration_lines,
            scenery_lines,
        })
    }
}

#[derive(Error, Debug)]
pub enum LineGroupBuilderError {
    #[error("Expected feature to be registered before passing feature data: {0}")]
    MissingFeatureFlag(LineFeature),
    #[error("Expected feature data to be present because feature was enabled: {0}")]
    MissingAttribute(&'static str),
    #[error("{0}")]
    StandardLineBuilderError(#[from] StandardLineBuilderError),
    #[error("{0}")]
    AccelerationLineBuilderError(#[from] AccelerationLineBuilderError),
    #[error("{0}")]
    SceneryLineBuilderError(#[from] SceneryLineBuilderError),
}
