use error::Error;

pub mod client;
// mod connection;
pub mod error;

pub type Result<T> = std::result::Result<T, Error>;
