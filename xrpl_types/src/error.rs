use crate::alloc::string::String;
use core::fmt;

/// Result type for xrpl types operations.
pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    InvalidData(String),
    OutOfRange(String),
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidData(s) => write!(f, "Value not valid in the given context: {}", s),
            Self::OutOfRange(s) => write!(f, "Value is out of range: {}", s),
        }
    }
}
