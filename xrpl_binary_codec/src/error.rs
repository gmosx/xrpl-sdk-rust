use std::num::ParseIntError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinaryCodecError {
    #[error("Error when parsing field: {0}")]
    ParseError(String),
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
