use thiserror::Error;

#[derive(Error, Debug)]
pub enum BinaryCodecError {
    #[error("Value not within the required range: {0}")]
    OutOfRange(String),
    #[error("Field order is wrong: {0}")]
    FieldOrder(String),
    #[error("Invalid field: {0}")]
    InvalidField(String),
}
