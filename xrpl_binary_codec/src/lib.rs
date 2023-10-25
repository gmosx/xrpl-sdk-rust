//! Binary serialization for XRPL Protocol objects.

pub mod deserializer;
mod error;
pub mod hash;
pub mod serialize;
#[allow(dead_code, unused_imports, unused_variables)]
pub mod deserialize;
/// Implements serialization according to <https://xrpl.org/serialization.html>
pub mod serializer;
pub mod sign;

pub use error::*;
