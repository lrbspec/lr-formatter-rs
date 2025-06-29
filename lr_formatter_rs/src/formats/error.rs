use crate::sol::{Amf0DeserializationError, Amf0SerializationError};
use crate::util::ParseLengthPrefixedStringError;
use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
};
use thiserror::Error;

// TODO: Replace String with &'static str

#[derive(Error, Debug)]
pub enum TrackReadError {
    #[error("{0}")]
    Io(#[from] io::Error),
    #[error("Invalid value for [{name}]: {value}")]
    InvalidData { name: String, value: String },
    #[error("{0}")]
    IntConversion(#[from] ParseIntError),
    #[error("{0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("{0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("{0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
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
