//! The account_tx method retrieves a list of transactions that involved the
//! specified account.
//!
//! <https://xrpl.org/account_tx.html>

use crate::{
    types::{Meta, Transaction},
    Request,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountTxRequest {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    // TODO: add more parameters!
}

impl Request for AccountTxRequest {
    type Response = AccountTxResponse;

    fn method(&self) -> String {
        "account_tx".to_owned()
    }
}

impl AccountTxRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }

    // #TODO add more builders.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            limit: Some(limit),
            ..self
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountTransaction {
    pub meta: Meta,
    // pub tx: serde_json::Value,
    pub tx: Transaction,
    pub validated: bool,
}

#[derive(Debug, Deserialize)]
pub struct AccountTxResponse {
    pub account: String,
    pub limit: u32,
    pub transactions: Vec<AccountTransaction>,
}
