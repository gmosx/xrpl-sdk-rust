//! The account_offers method retrieves a list of offers made by a given account
//! that are outstanding as of a particular ledger version.
//!
//! <https://xrpl.org/account_offers.html>

use crate::Request;
use serde::{Deserialize, Serialize};
use xrpl_types::Amount;

#[derive(Default, Clone, Serialize)]
pub struct AccountOffersRequest {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    strict: Option<bool>,
    // TODO: add more parameters!
}

impl Request for AccountOffersRequest {
    type Response = AccountOffersResponse;

    fn method(&self) -> String {
        "account_offers".to_owned()
    }
}

impl AccountOffersRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffer {
    pub flags: u32,
    pub quality: String,
    pub seq: u32,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountOffersResponse {
    pub offers: Vec<AccountOffer>,
}
