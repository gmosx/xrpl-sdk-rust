use error::Error;

pub mod client;
pub mod error;

mod util;

pub type Result<T> = std::result::Result<T, Error>;
