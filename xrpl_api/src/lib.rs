//! A low-level crate that extracts the common RPC protocol used by the HTML
//! and WebSocket clients. The data models defined in [`types`] and [`objects`] are
//! JSON representations compared to the crate `xrpl_types` that defines
//! the internal format of the types and objects.

mod api;
/// Notification events produced by <https://xrpl.org/subscribe.html>
mod events;
/// Objects on the XRP Ledger (<https://xrpl.org/ledger-object-types.html>).
mod objects;
/// Data types used on the XRP Ledger. This can be both basic data types
/// (<https://xrpl.org/basic-data-types.html>) and also complex data types used
/// in the API like transaction (<https://xrpl.org/transaction-formats.html>)
mod types;

pub use api::*;
pub use events::*;
pub use objects::*;
pub use types::*;
