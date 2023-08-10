//! <https://xrpl.org/ledger_entry.html>
//!
//! TIP: Better use the more specialized methods, like `get_offer_object`.

use crate::{
    LedgerObject, OfferParams, Request, RetrieveLedgerSpec, ReturnLedgerSpec, WithLedgerSpec,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct LedgerEntryRequest {
    #[serde(flatten)]
    pub ledger_spec: RetrieveLedgerSpec,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer: Option<OfferParams>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_root: Option<String>,
}

impl Request for LedgerEntryRequest {
    type Response = LedgerEntryResponse;

    fn method(&self) -> String {
        "ledger_entry".to_owned()
    }
}

impl WithLedgerSpec for LedgerEntryRequest {
    fn as_ledger_spec(&self) -> &crate::RetrieveLedgerSpec {
        &self.ledger_spec
    }

    fn as_ledger_spec_mut(&mut self) -> &mut crate::RetrieveLedgerSpec {
        &mut self.ledger_spec
    }
}

impl LedgerEntryRequest {
    pub fn offer(account: impl Into<String>, sequence: u32) -> Self {
        Self {
            offer: Some(OfferParams {
                account: account.into(),
                seq: sequence,
            }),
            ..Default::default()
        }
    }

    pub fn account(account: impl Into<String>) -> Self {
        Self {
            account_root: Some(account.into()),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LedgerEntryResponse {
    /// The unique ID of this ledger object.
    pub index: String,
    /// (Omitted if "binary": true specified.) Object containing the data of this ledger object, according to the ledger format.
    pub node: Option<LedgerObject>,
    /// (Omitted unless "binary":true specified) The binary representation of the ledger object, as hexadecimal.
    pub node_binary: Option<String>,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
