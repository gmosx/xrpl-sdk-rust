//! <https://xrpl.org/ledger_entry.html>
//!
//! TIP: Better use the more specialized methods, like `get_offer_object`.

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct OfferParams {
    account: String,
    seq: u32,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct LedgerEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    offer: Option<OfferParams>,
}

impl Request for LedgerEntryRequest {
    type Response = LedgerEntryResponse;

    fn method(&self) -> String {
        "ledger_entry".to_owned()
    }
}

impl LedgerEntryRequest {
    pub fn new(account: &str, sequence: u32) -> Self {
        Self {
            offer: Some(OfferParams {
                account: account.to_owned(),
                seq: sequence,
            }),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerEntryResponse {
    /// The unique ID of this ledger object.
    pub index: String,
    /// The ledger index of the ledger that was used when retrieving this data.
    pub ledger_index: u32,
    /// (Omitted if "binary": true specified.) Object containing the data of this ledger object, according to the ledger format.
    pub node: Option<serde_json::Value>,
    /// (Omitted unless "binary":true specified) The binary representation of the ledger object, as hexadecimal.
    pub node_binary: Option<String>,
}
