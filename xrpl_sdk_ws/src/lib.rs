pub mod api;
pub mod client;
pub mod error;

mod util;

use error::Error;

// #TODO move to `util`?
pub type Result<T> = std::result::Result<T, Error>;
