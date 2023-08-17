//! A strongly-typed client for the XRP Ledger HTTP JSONRPC API.

pub mod client;
pub mod error;

#[cfg(test)]
mod client_tests;

pub use client::{Client, Result};
pub use xrpl_api::*;
