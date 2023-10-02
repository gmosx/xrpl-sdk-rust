//! The fee command reports the current state of the open-ledger requirements
//! for the transaction cost.
//!
//! <https://xrpl.org/fee.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct FeeRequest {}

impl Request for FeeRequest {
    type Response = FeeResponse;

    fn method(&self) -> String {
        "fee".to_owned()
    }
}

impl FeeRequest {
    pub fn new() -> Self {
        Self::default()
    }
}

/// Various information about the transaction cost (the Fee field of a transaction),
/// in drops of XRP.
#[derive(Debug, Deserialize)]
pub struct Drops {
    /// The transaction cost required for a reference transaction to be included
    /// in a ledger under minimum load, represented in drops of XRP.
    pub base_fee: String,
    /// An approximation of the median transaction cost among transactions included
    /// in the previous validated ledger, represented in drops of XRP.
    pub median_fee: String,
    /// The minimum transaction cost for a reference transaction to be queued for
    /// a later ledger, represented in drops of XRP. If greater than base_fee,
    /// the transaction queue is full.
    pub minimum_fee: String,
    /// The minimum transaction cost that a reference transaction must pay to be
    /// included in the current open ledger, represented in drops of XRP.
    pub open_ledger_fee: String,
}

/// Various information about the transaction cost, in fee levels. The ratio
/// in fee levels applies to any transaction relative to the minimum cost of that
/// particular transaction.
#[derive(Debug, Deserialize)]
pub struct Levels {
    /// The median transaction cost among transactions in the previous validated
    /// ledger, represented in fee levels.
    pub median_level: String,
    /// The minimum transaction cost required to be queued for a future ledger,
    /// represented in fee levels.
    pub minimum_level: String,
    /// The minimum transaction cost required to be included in the current open
    /// ledger, represented in fee levels.
    pub open_ledger_level: String,
    /// The equivalent of the minimum transaction cost, represented in fee levels.
    pub reference_level: String,
}

#[derive(Debug, Deserialize)]
pub struct FeeResponse {
    /// Number of transactions provisionally included in the in-progress ledger.
    pub current_ledger_size: String,
    /// Number of transactions currently queued for the next ledger.
    pub current_queue_size: String,
    /// Various information about the transaction cost (the Fee field of a
    /// transaction), in drops of XRP.
    pub drops: Drops,
    /// The approximate number of transactions expected to be included in the current ledger. This is based on the number of transactions in the previous ledger.
    pub expected_ledger_size: String,
    /// The Ledger Index of the current open ledger these stats describe.
    pub ledger_current_index: u32,
    /// Various information about the transaction cost, in fee levels. The ratio
    /// in fee levels applies to any transaction relative to the minimum cost
    /// of that particular transaction.
    pub levels: Levels,
    /// The maximum number of transactions that the transaction queue can currently
    /// hold.
    pub max_queue_size: String,
}
