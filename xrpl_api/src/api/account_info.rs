//! <https://xrpl.org/account_info.html>

use crate::{AccountRoot, Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountInfoRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_lists: Option<bool>,
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

    // #TODO more builder methods
}

#[derive(Debug, Deserialize)]
pub struct AccountInfoResponse {
    // #TODO add missing fields!
    pub account_data: AccountRoot,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
