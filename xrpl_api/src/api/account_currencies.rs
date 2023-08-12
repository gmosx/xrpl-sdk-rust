//! <https://xrpl.org/account_currencies.html>

use serde::{Deserialize, Serialize};

use crate::{Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountCurrenciesRequest {
    pub account: String,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
}

impl Request for AccountCurrenciesRequest {
    type Response = AccountCurrenciesResponse;

    fn method(&self) -> String {
        "account_currencies".to_owned()
    }
}

impl WithLedgerSpec for AccountCurrenciesRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl AccountCurrenciesRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AccountCurrenciesResponse {
    /// Array of Currency Codes for currencies that this account can receive.
    pub receive_currencies: Vec<String>,
    /// Array of Currency Codes for currencies that this account can send.
    pub send_currencies: Vec<String>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
