//! The account_tx method retrieves a list of transactions that involved the
//! specified account.
//!
//! <https://xrpl.org/account_tx.html>

use crate::{
    types::{Meta, Transaction},
    Request, RequestPagination, ResponsePagination, RetrieveLedgerSpec, ReturnLedgerSpec,
    WithLedgerSpec, WithRequestPagination, WithResponsePagination,
};
use serde::{Deserialize, Serialize};
use xrpl_types::LedgerIndex;

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountTxRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index_max: Option<String>,
    pub forward: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<LedgerIndex>,
    #[serde(flatten)]
    pub pagination: RequestPagination,
}

impl Request for AccountTxRequest {
    type Response = AccountTxResponse;

    fn method(&self) -> String {
        "account_tx".to_owned()
    }
}

impl WithRequestPagination for AccountTxRequest {
    fn as_pagination(&self) -> &RequestPagination {
        &self.pagination
    }

    fn as_pagination_mut(&mut self) -> &mut RequestPagination {
        &mut self.pagination
    }
}

impl AccountTxRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
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
    pub ledger_index_min: u32,
    pub ledger_index_max: u32,
    pub transactions: Vec<AccountTransaction>,
    pub validated: bool,
    #[serde(flatten)]
    pub pagination: ResponsePagination,
}

impl WithResponsePagination for AccountTxResponse {
    fn as_pagination(&self) -> &ResponsePagination {
        &self.pagination
    }
}
