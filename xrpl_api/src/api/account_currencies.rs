//! https://xrpl.org/account_currencies.html

use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountCurrenciesRequest {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountCurrenciesResponse {
    /// The ledger index of the ledger version used to retrieve this data.
    pub ledger_index: u32,
    /// Array of Currency Codes for currencies that this account can receive.
    pub receive_currencies: Vec<String>,
    /// Array of Currency Codes for currencies that this account can send.
    pub send_currencies: Vec<String>,
    /// If true, this data comes from a validated ledger.
    pub validated: bool,
}
