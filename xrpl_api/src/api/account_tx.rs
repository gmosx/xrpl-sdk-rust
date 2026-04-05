//! The account_tx method retrieves a list of transactions that involved the
//! specified account.
//!
//! <https://xrpl.org/account_tx.html>

use crate::{
    types::{Meta, Transaction},
    LedgerIndex, Request, RequestPagination, ResponsePagination, WithRequestPagination,
    WithResponsePagination,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountTxRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index_max: Option<String>,
    pub forward: Option<bool>,

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

// BASELINE: test that AccountTxResponse fails when any transaction has unknown type — see issue #41
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_account_tx_response_fails_on_unknown_transaction_type() {
        // A response with one known and one unknown transaction type.
        // Currently FAILS — the Vec<AccountTransaction> cannot deserialize
        // when any element contains an unknown TransactionType.
        let json = r#"{
            "account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
            "ledger_index_min": 1,
            "ledger_index_max": 100,
            "validated": true,
            "transactions": [
                {
                    "meta": {
                        "AffectedNodes": [],
                        "TransactionIndex": 0,
                        "TransactionResult": "tesSUCCESS"
                    },
                    "validated": true,
                    "tx": {
                        "TransactionType": "OracleSet",
                        "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                        "Fee": "12",
                        "Sequence": 1
                    }
                }
            ]
        }"#;
        // After fix: this should succeed — OracleSet deserializes as Transaction::OracleSet(...)
        let _: AccountTxResponse = serde_json::from_str(json).unwrap();
    }
}
