use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinaryCodecError {
    #[error("Error when parsing field: {0}")]
    ParseError(String),
    #[error("Value not within the required range: {0}")]
    OutOfRange(String),
    #[error("Field order is wrong: {0}")]
    FieldOrder(String),
    #[error("IO error: {0}")]
    IO(String),
    #[error("Invalid field: {0}")]
    InvalidField(String),
}

impl From<bs58::decode::Error> for BinaryCodecError {
    fn from(_: bs58::decode::Error) -> Self {
        BinaryCodecError::ParseError("Unable to decode from bs58".to_string())
    }
}

impl From<ParseIntError> for BinaryCodecError {
    fn from(_: ParseIntError) -> Self {
        BinaryCodecError::ParseError("Unable to parse integer from string".to_string())
    }
}

impl From<std::io::Error> for BinaryCodecError {
    fn from(error: std::io::Error) -> Self {
        BinaryCodecError::IO(format!("IO error: {}", error))
    }
}
