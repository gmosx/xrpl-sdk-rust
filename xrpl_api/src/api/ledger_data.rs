//! The ledger_data method retrieves contents of the specified ledger. You can
//! iterate through several calls to retrieve the entire contents of a single
//! ledger version.
//!
//! <https://xrpl.org/ledger_data.html>

use crate::Request;
use serde::{Deserialize, Serialize};

// TIP: Better use the more specialized methods, like `get_offer_object`.

#[derive(Default, Debug, Clone, Serialize)]
pub struct LedgerDataRequest {
    /// A 20-byte hex string for the ledger version to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    /// The ledger index of the ledger to use, or a shortcut string to choose a ledger automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    /// If set to true, return ledger objects as hashed hex strings instead of JSON.
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    /// Limit the number of ledger objects to retrieve. The server is not
    /// required to honor this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// Value from a previous paginated response. Resume retrieving data where
    /// that response left off.
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
}

impl Request for LedgerDataRequest {
    type Response = LedgerDataResponse;

    fn method(&self) -> String {
        "ledger_data".to_owned()
    }
}

impl LedgerDataRequest {
    // #TODO force either ledger_hash or ledger_index
    pub fn new(ledger_hash: &str) -> Self {
        Self {
            ledger_hash: Some(ledger_hash.to_owned()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LedgerData {
    /// Hex representation of the requested data.
    /// (Only included if "binary":true)
    pub data: Option<String>,
    #[serde(rename = "LedgerEntryType")]
    /// String indicating what type of ledger object this object represents.
    /// (Only included if "binary":false)
    pub ledger_entry_type: Option<String>,
    /// Unique identifier for this ledger entry, as hex.
    pub index: String,
}

#[derive(Debug, Deserialize)]
pub struct LedgerDataResponse {
    /// The ledger index of the ledger that was used when retrieving this data.
    pub ledger_index: u32,
    /// Unique identifying hash of this ledger version.
    pub ledger_hash: String,
    /// Array of JSON objects containing data from the ledger's state tree.
    pub state: Vec<LedgerData>,
    /// Server-defined value indicating the response is paginated. Pass this to
    /// the next call to resume where this call left off.
    pub marker: Option<String>,
}
