use crate::Request;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use xrpl_types::Amount;

/// The gateway_balances command calculates the total balances issued by a
/// given account, optionally excluding amounts held by operational addresses.
///
/// -https://xrpl.org/gateway_balances
#[derive(Default, Clone, Serialize)]
pub struct GatewayBalancesRequest {
    /// The address to check. This should be the issuing address.
    pub account: String,
    /// An operational address to exclude from the balances issued, or an array
    /// of such addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotwallet: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl Request for GatewayBalancesRequest {
    type Response = GatewayBalancesResponse;

    fn method(&self) -> String {
        "gateway_balances".to_owned()
    }
}

impl GatewayBalancesRequest {
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
pub struct GatewayBalancesResponse {
    /// The address of the account that issued the balances.
    pub account: String,
    pub obligations: Option<HashMap<String, String>>,
    pub balances: Option<HashMap<String, Vec<Amount>>>,
    pub assets: Option<HashMap<String, Vec<Amount>>>,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<u32>,
    pub ledger_current_index: Option<u32>,
}
