//! The ripple_path_find method is a simplified version of the path_find method
//! that provides a single response with a payment path you can use right away.
//! It is available in both the WebSocket and JSON-RPC APIs. However, the
//! results tend to become outdated as time passes. Instead of making multiple
//! calls to stay updated, you should instead use the path_find method to
//! subscribe to continued updates where possible.
//!
//! Although the rippled server tries to find the cheapest path or combination
//! of paths for making a payment, it is not guaranteed that the paths
//! returned by this method are, in fact, the best paths.
//!
//! <https://xrpl.org/ripple_path_find.html>
//!
//! <https://xrpl.org/paths.html>

use serde::{Deserialize, Serialize};
use serde_json::Value;
use xrpl_types::Amount;

use crate::Request;

// #TODO is Clone really needed?
#[derive(Default, Debug, Clone, Serialize)]
pub struct RipplePathFindRequest {
    /// Unique address of the account that would send funds in a transaction.
    source_account: String,
    /// Unique address of the account that would receive funds in a transaction.
    destination_account: String,
    /// Currency Amount that the destination account would receive in a
    /// transaction. Special case: New in: rippled 0.30.0  You can specify "-1"
    /// (for XRP) or provide -1 as the contents of the value field
    /// (for non-XRP currencies). This requests a path to deliver as much as
    /// possible, while spending no more than the amount specified in
    /// send_max (if provided).
    destination_amount: Amount,
    #[serde(skip_serializing_if = "Option::is_none")]
    send_max: Option<Amount>,
    /// A 20-byte hex string for the ledger version to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    /// The ledger index of the ledger to use, or a shortcut string to choose a
    /// ledger automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
}

impl Request for RipplePathFindRequest {
    type Response = RipplePathFindResponse;

    fn method(&self) -> String {
        "ripple_path_find".to_owned()
    }
}

impl RipplePathFindRequest {
    pub fn new(
        source_account: &str,
        destination_account: &str,
        destination_amount: Amount,
    ) -> Self {
        Self {
            source_account: source_account.to_owned(),
            destination_account: destination_account.to_owned(),
            destination_amount,
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Path {
    // #TODO full types missing
    /// Array of arrays of objects defining payment paths.
    pub paths_computed: Vec<Vec<Value>>,
    /// Currency Amount that the source would have to send along this path for
    /// the destination to receive the desired amount.
    pub source_amount: Amount,
}

#[derive(Debug, Deserialize)]
pub struct RipplePathFindResponse {
    pub alternatives: Vec<Path>,
    pub destination_account: String,
    pub destination_currencies: Vec<String>,
}
