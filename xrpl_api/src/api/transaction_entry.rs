//! The transaction_entry method retrieves information on a single transaction
//! from a specific ledger version. (The tx method, by contrast, searches all
//! ledgers for the specified transaction. We recommend using that method
//! instead.)
//!
//! https://xrpl.org/transaction_entry.html

use crate::{types::Meta, types::Transaction, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct TransactionEntryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    /// Unique hash of the transaction you are looking up.
    tx_hash: String,
}

impl Request for TransactionEntryRequest {
    type Response = TransactionEntryResponse;

    fn method(&self) -> String {
        "transaction_entry".to_owned()
    }
}

impl TransactionEntryRequest {
    pub fn new(tx_hash: &str) -> Self {
        Self {
            tx_hash: tx_hash.to_owned(),
            ..Default::default()
        }
    }

    pub fn ledger_index(self, ledger_index: &str) -> Self {
        Self {
            ledger_index: Some(ledger_index.to_owned()),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TransactionEntryResponse {
    /// The ledger index of the ledger version the transaction was found in; this is the same as the one from the request.
    pub ledger_index: u32,
    /// The identifying hash of the ledger version the transaction was found in; this is the same as the one from the request.
    pub ledger_hash: Option<String>,
    pub metadata: Meta,
    pub tx_json: Transaction,
}
