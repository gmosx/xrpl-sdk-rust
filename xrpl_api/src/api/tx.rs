//! The `tx` method retrieves information on a single transaction, by its
//! identifying hash.
//!
//! <https://xrpl.org/tx.html>

use crate::{types::Meta, types::Transaction, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct TxRequest {
    /// The 256-bit hash of the transaction, as hex.
    transaction: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_ledger: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_ledger: Option<u32>,
}

impl Request for TxRequest {
    type Response = TxResponse;

    fn method(&self) -> String {
        "tx".to_owned()
    }
}

impl TxRequest {
    pub fn new(transaction: &str) -> Self {
        Self {
            transaction: transaction.to_owned(),
            ..Default::default()
        }
    }

    pub fn binary(self, binary: bool) -> Self {
        Self {
            binary: Some(binary),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct TxResponse {
    /// The SHA-512 hash of the transaction.
    // pub hash: String,
    /// The ledger index of the ledger that includes this transaction.
    pub ledger_index: u32,
    /// If true, this data comes from a validated ledger version; if omitted or
    /// set to false, this data is not final.
    pub validated: bool,
    /// Transaction metadata, which describes the results of the transaction.
    pub meta: Meta,
    /// Other fields from the Transaction object.
    #[serde(flatten)]
    pub tx: Transaction,
}
