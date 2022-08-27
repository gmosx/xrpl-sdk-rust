//! The deposit_authorized command indicates whether one account is authorized
//! to send payments directly to another. See Deposit Authorization for
//! information on how to require authorization to deliver money to your account.
//!
//! <https://xrpl.org/deposit_authorized.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct DepositAuthorizedRequest {
    /// The sender of a possible payment.
    source_account: String,
    /// The recipient of a possible payment.
    destination_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
}

impl Request for DepositAuthorizedRequest {
    type Response = DepositAuthorizedResponse;

    fn method(&self) -> String {
        "deposit_authorized".to_owned()
    }
}

impl DepositAuthorizedRequest {
    pub fn new(source_account: &str, destination_account: &str) -> Self {
        Self {
            source_account: source_account.to_owned(),
            destination_account: destination_account.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct DepositAuthorizedResponse {
    /// Whether the specified source account is authorized to send payments
    /// directly to the destination account. If true, either the destination
    /// account does not require Deposit Authorization or the source account is
    /// preauthorized.
    pub deposit_authorized: bool,
    pub source_account: String,
    pub destination_account: String,
    pub ledger_hash: Option<String>,
    pub ledger_index: Option<u32>,
    pub ledger_current_index: Option<u32>,
}
