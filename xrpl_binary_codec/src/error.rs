use std::{fmt, num::ParseIntError};

#[derive(Debug)]
pub enum BinaryCodecError {
    ParseError(String),
}

impl std::error::Error for BinaryCodecError {}

impl fmt::Display for BinaryCodecError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BinaryCodecError::ParseError(reason) => {
                write!(f, "Error when parsing field {}", reason)
            }
        }
    }
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
