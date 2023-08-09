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

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountTxRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index_max: Option<String>,
    pub forward: Option<bool>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
    #[serde(flatten)]
    pub pagination: RequestPagination,
}

impl Request for AccountTxRequest {
    type Response = AccountTxResponse;

    fn method(&self) -> String {
        "account_tx".to_owned()
    }
}

impl WithLedgerSpec for AccountTxRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
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
    pub transactions: Vec<AccountTransaction>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
    #[serde(flatten)]
    pub pagination: ResponsePagination,
}

impl WithResponsePagination for AccountTxResponse {
    fn as_pagination(&self) -> &ResponsePagination {
        &self.pagination
    }
}
