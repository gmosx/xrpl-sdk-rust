//! Binary serialization for XRPL Protocol objects.

#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

pub mod deserializer;
mod error;
pub mod hash;
pub mod serialize;
/// Implements serialization according to <https://xrpl.org/serialization.html>
pub mod serializer;
pub mod sign;

pub use error::*;
