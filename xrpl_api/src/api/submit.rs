//! The submit method applies a transaction and sends it to the network to be
//! confirmed and included in future ledgers.
//!
//! This command has two modes:
//!
//! Submit-only mode takes a signed, serialized transaction as a binary blob,
//! and submits it to the network as-is. Since signed transaction objects are
//! immutable, no part of the transaction can be modified or automatically
//! filled in after submission.
//!
//! Sign-and-submit mode takes a JSON-formatted Transaction object, completes
//! and signs the transaction in the same manner as the sign method, and then
//! submits the signed transaction. We recommend only using this mode for
//! testing and development.
//!
//! To send a transaction as robustly as possible, you should construct and
//! sign it in advance, persist it somewhere that you can access even after a
//! power outage, then submit it as a tx_blob. After submission, monitor the
//! network with the tx method command to see if the transaction was
//! successfully applied; if a restart or other problem occurs, you can safely
//! re-submit the tx_blob transaction: it won't be applied twice since it has
//! the same sequence number as the old transaction.
//!
//! - https://xrpl.org/submit.html

use crate::Request;
use serde::Serialize;

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
