use std::{
    io,
    num::{ParseFloatError, ParseIntError, TryFromIntError},
};
use thiserror::Error;

use crate::sol::{Amf0DeserializationError, Amf0SerializationError};
use crate::util::ParseLengthPrefixedStringError;

#[derive(Error, Debug)]
pub enum TrackReadError {
    #[error("IO error while reading track file: {0}")]
    Io(#[from] io::Error),
    #[error("Invalid value for [{name}] while reading track file: {value}")]
    InvalidData { name: String, value: String },
    #[error("Int conversion error while reading track file: {0}")]
    IntConversion(#[from] ParseIntError),
    #[error("Float conversion error while reading track file: {0}")]
    FloatConversion(#[from] ParseFloatError),
    #[error("String parsing error while reading track file: {0}")]
    StringParsing(#[from] ParseLengthPrefixedStringError),
    #[error("Amf0 error while reading track file: {0}")]
    Amf0Deserialization(#[from] Amf0DeserializationError),
    #[error("Other error while reading track file: {message}")]
    Other { message: String },
}

#[derive(Error, Debug)]
pub enum TrackWriteError {
    #[error("IO error while writing track file: {0}")]
    Io(#[from] io::Error),
    #[error("Int cast error while reading track file: {0}")]
    IntConversion(#[from] TryFromIntError),
    #[error("Amf0 error while writing track file: {0}")]
    Amf0Serialization(#[from] Amf0SerializationError),
    #[error("Other error while reading track file: {message}")]
    Other { message: String },
}
