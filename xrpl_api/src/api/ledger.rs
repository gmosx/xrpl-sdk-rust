//! <https://xrpl.org/ledger.html>

use crate::{types::Transaction, AccountTransaction, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct LedgerTransactionsRequest {
    #[serde(flatten)]
    pub LedgerRequest: LedgerRequest,
}

#[derive(Default, Clone, Serialize)]
pub struct LedgerExpandRequest {
    #[serde(flatten)]
    pub LedgerRequest: LedgerRequest,
}

#[derive(Default, Clone, Serialize)]
pub struct LedgerRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    /// The ledger index of the ledger to use, or a shortcut string to choose a ledger automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    full: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    accounts: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transactions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    expand: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    owner_funds: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    queue: Option<bool>,
}

impl Request for LedgerRequest {
    type Response = Hash;

    fn method(&self) -> String {
        "ledger".to_owned()
    }
}

impl LedgerTransactionsRequest {
    pub fn new(ledger_index: &str, transactions: bool) -> Self {
        Self {
            LedgerRequest: LedgerRequest {
                ledger_index: Some(ledger_index.to_owned()),
                transactions: Some(transactions),
                expand: Some(false),
                ..Default::default()
            },
        }
    }
}

impl Request for LedgerExpandTransactionsRequest {
    type Response = Expand;

    fn method(&self) -> String {
        "ledger".to_owned()
    }
}

impl LedgerExpandTransactionsRequest {
    pub fn new(ledger_index: &str, transactions: bool) -> Self {
        Self {
            LedgerRequest: LedgerRequest {
                ledger_index: Some(ledger_index.to_owned()),
                transactions: Some(transactions),
                expand: Some(true),
                ..Default::default()
            },
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerResponse<TransactionType> {
    ///Unique identifying hash of the entire ledger.
    pub ledger_hash: String,
    ///The Ledger Index of this ledger.
    pub ledger_index: u32,
    ///(May be omitted) If true, this is a validated ledger version. If omitted or set to false, this ledger's data is not final.
    pub validated: bool,
    ///(Omitted unless requested with the queue parameter) Array of objects describing queued transactions, in the same order as the queue.
    ///If the request specified expand as true, members contain full representations of the transactions, in either JSON or binary depending on whether the request specified binary as true.
    pub queue_data: Option<u64>,
    ///The complete header data of this ledger.
    pub ledger: Ledger<TransactionType>,
}

#[derive(Debug, Deserialize)]
pub struct Ledger<TransactionType> {
    ///Hash of all account state information in this ledger, as hex.
    pub account_hash: String,
    pub close_flags: u64,
    ///The time this ledger was closed, in seconds since the Ripple Epoch.
    pub close_time: u64,
    ///The time this ledger was closed, in human-readable format. Always uses the UTC time zone.
    pub close_time_human: String,
    ///Ledger close times are rounded to within this many seconds.
    pub close_time_resolution: u64,
    ///Whether or not this ledger has been closed.
    pub closed: bool,
    ///Unique identifying hash of the entire ledger.
    pub ledger_hash: String,
    ///The Ledger Index of this ledger, as a quoted integer.
    pub ledger_index: String,
    ///The time at which the previous ledger was closed.
    pub parent_close_time: u64,
    ///Unique identifying hash of the ledger that came immediately before this one.
    pub parent_hash: String,
    ///Total number of XRP drops in the network, as a quoted integer. (This decreases as transaction costs destroy XRP.)
    pub total_coins: String,
    ///Hash of the transaction information included in this ledger, as hex
    pub transaction_hash: String,
    ///(Omitted unless requested) Transactions applied in this ledger version.
    ///By default, members are the transactions' identifying Hash strings. If the request specified expand as true,
    ///members are full representations of the transactions instead,
    ///in either JSON or binary depending on whether the request specified binary as true.
    pub transactions: Option<Vec<TransactionType>>,
}

type Expand = LedgerResponse<Transaction>;
type Hash = LedgerResponse<String>;
