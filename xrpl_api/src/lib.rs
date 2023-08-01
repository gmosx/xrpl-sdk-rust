//! A low-level crate that extracts the common RPC protocol used by the HTML
//! and WebSocket clients.

mod api;
pub mod types;

pub use api::*;
pub use types::Request;
