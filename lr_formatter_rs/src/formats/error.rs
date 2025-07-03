use crate::formats::sol::{Amf0DeserializationError, Amf0SerializationError};
use crate::track::TrackBuilderError;
use crate::track::layer::layer_group::LayerGroupBuilderError;
use crate::track::line::line_group::LineGroupBuilderError;
use crate::track::rider::rider_group::RiderGroupBuilderError;
use crate::util::ParseLengthPrefixedStringError;
use std::string::FromUtf8Error;
use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TrackReadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    // TODO maybe remove this
    #[error("Invalid value for `{name}`: {value}")]
    InvalidData { name: String, value: String },
    #[error("{0}")]
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("{0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
    #[error("{0}")]
    TrackBuilderError(#[from] TrackBuilderError),
    #[error("{0}")]
    LineGroupBuilderError(#[from] LineGroupBuilderError),
    #[error("{0}")]
    RiderGroupBuilderError(#[from] RiderGroupBuilderError),
    #[error("{0}")]
    LayerGroupBuilderError(#[from] LayerGroupBuilderError),
    #[error("{0}")]
    FromUTF8Error(#[from] FromUtf8Error),
    // TODO remove this
    #[error("{message}")]
    Other { message: String },
}

#[derive(Error, Debug)]
pub enum TrackWriteError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("{0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("{0}")]
    Amf0Serialization(#[from] Amf0SerializationError),
    // TODO remove this
    #[error("{message}")]
    Other { message: String },
}
