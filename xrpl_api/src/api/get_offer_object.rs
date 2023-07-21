//! <https://xrpl.org/ledger_entry.html>
//!
//! <https://xrpl.org/ledger_entry.html#get-offer-object>

use serde::{Deserialize, Serialize};
use xrpl_types::Offer;

use crate::Request;

#[derive(Default, Debug, Clone, Serialize)]
pub struct OfferParams {
    account: String,
    seq: u32,
}

#[derive(Default, Clone, Serialize)]
pub struct GetOfferObjectRequest {
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary: Option<bool>,
    offer: OfferParams,
}

impl Request for GetOfferObjectRequest {
    type Response = GetOfferObjectResponse;

    fn method(&self) -> String {
        "ledger_entry".to_owned()
    }
}

impl GetOfferObjectRequest {
    pub fn new(account: &str, sequence: u32) -> Self {
        Self {
            offer: OfferParams {
                account: account.to_owned(),
                seq: sequence,
            },
            ..Default::default()
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct GetOfferObjectResponse {
    /// The unique ID of this ledger object.
    pub index: String,
    /// The ledger index of the ledger that was used when retrieving this data.
    pub ledger_index: u32,
    /// (Omitted if "binary": true specified.) Object containing the data of this ledger object, according to the ledger format.
    pub node: Option<Offer>,
    /// (Omitted unless "binary":true specified) The binary representation of the ledger object, as hexadecimal.
    pub node_binary: Option<String>,
}
