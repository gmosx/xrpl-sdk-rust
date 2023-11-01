//! Binary serialization for XRPL Protocol objects.
#![allow(stable_features)]
#![feature(once_cell)]

pub mod deserializer;
mod error;
pub mod hash;
pub mod serialize;
/// Implements serialization according to <https://xrpl.org/serialization.html>
pub mod serializer;
pub mod sign;

pub use error::*;
