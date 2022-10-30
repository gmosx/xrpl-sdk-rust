//! <https://xrpl.org/account_info.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountInfoRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl Request for AccountInfoRequest {
    type Response = AccountInfoResponse;

    fn method(&self) -> String {
        "account_info".to_owned()
    }
}

impl AccountInfoRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }

    pub fn strict(self, strict: bool) -> Self {
        Self {
            strict: Some(strict),
            ..self
        }
    }

    pub fn ledger_index(self, ledger_index: &str) -> Self {
        Self {
            ledger_index: Some(ledger_index.to_owned()),
            ..self
        }
    }

    // #TODO more builder methods
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    // TODO!
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "Balance")]
    pub balance: String,

    #[serde(rename = "Sequence")]
    pub sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoResponse {
    // #TODO add missing fields!
    pub account_data: AccountData,
    pub ledger_current_index: Option<u64>,
    pub ledger_index: Option<u64>,
}
