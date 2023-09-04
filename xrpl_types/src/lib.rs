//! Core types and related functions for the XRP Ledger. Reused between Web and
//! WebSocket clients and potentially for server-side code.

mod error;
mod ledger_timestamp;
pub mod serialize;
/// Types in internal canonical binary format <https://xrpl.org/serialization.html#type-list>
mod types;

pub use error::*;
pub use ledger_timestamp::*;
pub use types::*;
