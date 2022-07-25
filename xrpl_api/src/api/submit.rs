use crate::Request;
use serde::Serialize;

/// - https://xrpl.org/submit.html
#[derive(Default, Clone, Serialize)]
pub struct SubmitRequest {
    /// Hex representation of the signed transaction to submit. This can be a
    /// multi-signed transaction.
    tx_blob: String,
    /// (Optional, defaults to false) If true, and the transaction fails locally,
    /// do not retry or relay the transaction to other servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    fail_hard: Option<bool>,
}

impl Request for SubmitRequest {
    type Response = SubmitResponse;

    fn method(&self) -> String {
        "submit".to_owned()
    }
}

impl SubmitRequest {
    pub fn new(tx_blob: &str) -> Self {
        Self {
            tx_blob: tx_blob.to_owned(),
            ..Default::default()
        }
    }
}

// #[derive(Debug, Deserialize)]
// pub struct SubmitResponse {
//     pub accepted: Option<bool>,
//     pub applied: bool,
//     pub engine_result: String,
//     pub status: String,
// }

// #TODO implement typed response
pub type SubmitResponse = serde_json::Value;
