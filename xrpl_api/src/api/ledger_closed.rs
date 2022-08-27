//! The ledger_closed method returns the unique identifiers of the most recently
//! closed ledger. (This ledger is not necessarily validated and immutable yet.)
//!
//! <https://xrpl.org/ledger_closed.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct LedgerClosedRequest {}

impl Request for LedgerClosedRequest {
    type Response = LedgerClosedResponse;

    fn method(&self) -> String {
        "ledger_closed".to_owned()
    }
}

impl LedgerClosedRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerClosedResponse {
    /// The unique Hash of this ledger version, in hexadecimal.
    pub ledger_hash: String,
    /// The ledger index of this ledger version.
    pub ledger_index: u32,
}
