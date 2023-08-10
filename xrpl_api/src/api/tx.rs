//! The `tx` method retrieves information on a single transaction, by its
//! identifying hash.
//!
//! <https://xrpl.org/tx.html>

use crate::{types::Meta, types::Transaction, Request};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
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
    #[serde(flatten)]
    pub tx: Transaction,
}
