use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinaryCodecError {
    #[error("Value not within the required range: {0}")]
    OutOfRange(String),
    #[error("Field order is wrong: {0}")]
    FieldOrder(String),
    #[error("Invalid field: {0}")]
    InvalidField(String),
    #[error("Invalid length: {0}")]
    InvalidLength(String),
    #[error("Field not found: {0}")]
    FieldNotFound(String),
    #[error("Insufficient bytes to decode: {0}")]
    InsufficientBytes(String),
    #[error("Bytes overflow")]
    Overflow,
}
