pub mod client;
pub mod error;

#[cfg(test)]
mod client_tests;

pub use client::{Client, Result};
pub use xrpl_api::api::*;
