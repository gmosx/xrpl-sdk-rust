//! The gateway_balances command calculates the total balances issued by a
//! given account, optionally excluding amounts held by operational addresses.
//!
//! <https://xrpl.org/gateway_balances>

use crate::{Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use xrpl_types::Amount;

#[derive(Default, Debug, Clone, Serialize)]
pub struct GatewayBalancesRequest {
    /// The address to check. This should be the issuing address.
    pub account: String,
    /// An operational address to exclude from the balances issued, or an array
    /// of such addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hotwallet: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
}

impl Request for GatewayBalancesRequest {
    type Response = GatewayBalancesResponse;

    fn method(&self) -> String {
        "gateway_balances".to_owned()
    }
}

impl WithLedgerSpec for GatewayBalancesRequest {
    fn as_ledger_spec(&self) -> &RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut RetrieveLedgerSpec {
        &mut self.ledger_spec
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

#[derive(Debug, Deserialize)]
pub struct GatewayBalancesResponse {
    /// The address of the account that issued the balances.
    pub account: String,
    pub obligations: Option<HashMap<String, String>>,
    pub balances: Option<HashMap<String, Vec<Amount>>>,
    pub assets: Option<HashMap<String, Vec<Amount>>>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
