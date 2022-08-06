//! A strongly-typed client for the XRP Ledger WebSocket API.

pub mod client;
pub mod error;

mod util;

#[cfg(test)]
mod client_tests;

pub use util::Result;
