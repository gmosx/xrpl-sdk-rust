//! <https://xrpl.org/ledger.html>

use crate::{types::Transaction, Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec};
use serde::{Deserialize, Serialize};
use xrpl_types::LedgerTimestamp;

// #TODO refactor to make the two variants internal!
// #TODO add tests

#[derive(Default, Debug, Clone, Serialize)]
pub struct ExpandLedgerRequest {
    #[serde(flatten)]
    pub ledger_request: LedgerRequest,
}

#[derive(Default, Debug, Clone, Serialize)]
pub struct LedgerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_funds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<bool>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub object_type: Option<ObjectType>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
}

#[derive(Clone, Debug, Copy, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObjectType {
    Account,
    Amendments,
    Amm,
    Check,
    DepositPreauth,
    Directory,
    Escrow,
    Fee,
    Hashes,
    NftOffer,
    NftPage,
    Offer,
    PaymentChannel,
    SignerList,
    State,
    Ticket,
}

impl Request for LedgerRequest {
    type Response = LedgerResponse<String>;

    fn method(&self) -> String {
        "ledger".to_owned()
    }
}

impl WithLedgerSpec for LedgerRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl LedgerRequest {
    pub fn new() -> Self {
        Self {
            expand: Some(false),
            ..Default::default()
        }
    }

    pub fn transactions(self, transactions: bool) -> Self {
        Self {
            transactions: Some(transactions),
            ..self
        }
    }

    pub fn expanded(self) -> ExpandLedgerRequest {
        ExpandLedgerRequest {
            ledger_request: LedgerRequest {
                expand: Some(true),
                ..self
            },
        }
    }
}

impl Request for ExpandLedgerRequest {
    type Response = LedgerResponse<Transaction>;

    fn method(&self) -> String {
        "ledger".to_owned()
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerResponse<TransactionType> {
    /// (Omitted unless requested with the queue parameter) Array of objects describing queued transactions, in the same order as the queue.
    /// If the request specified expand as true, members contain full representations of the transactions, in either JSON or binary depending on whether the request specified binary as true.
    // pub queue_data: Vec<???>,
    /// The complete header data of this ledger.
    pub ledger: Ledger<TransactionType>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}

#[derive(Debug, Deserialize)]
pub struct Ledger<TransactionType> {
    /// Hash of all account state information in this ledger, as hex.
    pub account_hash: String,
    pub close_flags: u64,
    /// The time this ledger was closed, in seconds since the Ripple Epoch.
    pub close_time: LedgerTimestamp,
    /// The time this ledger was closed, in human-readable format. Always uses the UTC time zone.
    pub close_time_human: String,
    /// Ledger close times are rounded to within this many seconds.
    pub close_time_resolution: u32,
    /// Whether or not this ledger has been closed.
    pub closed: bool,
    /// Unique identifying hash of the entire ledger.
    pub ledger_hash: String,
    /// The Ledger Index of this ledger, as a quoted integer.
    pub ledger_index: String,
    /// The time at which the previous ledger was closed.
    pub parent_close_time: LedgerTimestamp,
    /// Unique identifying hash of the ledger that came immediately before this one.
    pub parent_hash: String,
    /// Total number of XRP drops in the network, as a quoted integer. (This decreases as transaction costs destroy XRP.)
    pub total_coins: String,
    /// Hash of the transaction information included in this ledger, as hex
    pub transaction_hash: String,
    /// (Omitted unless requested) Transactions applied in this ledger version.
    /// By default, members are the transactions identifying Hash strings. If the request specified expand as true,
    /// members are full representations of the transactions instead,
    /// in either JSON or binary depending on whether the request specified binary as true.
    pub transactions: Option<Vec<TransactionType>>,
}
