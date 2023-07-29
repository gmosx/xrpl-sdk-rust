//! <https://xrpl.org/account_info.html>

use crate::{Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountInfoRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
}

impl Request for AccountInfoRequest {
    type Response = AccountInfoResponse;

    fn method(&self) -> String {
        "account_info".to_owned()
    }
}

impl WithLedgerSpec for AccountInfoRequest {
    fn as_ledger_spec(&self) -> &RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut RetrieveLedgerSpec {
        &mut self.ledger_spec
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

#[derive(Debug, Deserialize)]
pub struct AccountInfoResponse {
    // #TODO add missing fields!
    pub account_data: AccountData,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
