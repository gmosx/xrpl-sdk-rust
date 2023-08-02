use thiserror::Error;

// todo allan
#[derive(Error, Debug)]
pub enum Error {
    #[error("Value not valid in the given context: {0}")]
    InvalidData(String),
}
