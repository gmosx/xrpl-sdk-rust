use crate::Request;
use serde::{Deserialize, Serialize};

/// The ledger_current method returns the unique identifiers of the current
/// in-progress ledger. This command is mostly useful for testing, because the
/// ledger returned is still in flux.
///
/// - https://xrpl.org/ledger_closed.html
#[derive(Default, Clone, Serialize)]
pub struct LedgerCurrentRequest {}

impl Request for LedgerCurrentRequest {
    type Response = LedgerCurrentResponse;

    fn method(&self) -> String {
        "ledger_current".to_owned()
    }
}

impl LedgerCurrentRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerCurrentResponse {
    /// The ledger index of this ledger version..
    pub ledger_current_index: u32,
}
