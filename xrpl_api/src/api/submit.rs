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
//! <https://xrpl.org/submit.html>

use crate::{Request, Transaction, TransactionResult};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct SubmitRequest {
    /// Hex representation of the signed transaction to submit. This can be a
    /// multi-signed transaction.
    pub tx_blob: String,
    /// (Optional, defaults to false) If true, and the transaction fails locally,
    /// do not retry or relay the transaction to other servers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fail_hard: Option<bool>,
}

impl Request for SubmitRequest {
    type Response = SubmitResponse;

    fn method(&self) -> String {
        "submit".to_owned()
    }
}

impl SubmitRequest {
    pub fn new(tx_blob: impl Into<String>) -> Self {
        Self {
            tx_blob: tx_blob.into(),
            ..Default::default()
        }
    }

    pub fn fail_hard(self, fail_hard: bool) -> Self {
        Self {
            fail_hard: Some(fail_hard),
            ..self
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct SubmitResponse {
    pub engine_result: TransactionResult,
    pub engine_result_code: i32,
    pub engine_result_message: String,
    pub tx_blob: String,
    pub tx_json: Transaction,
    pub accepted: bool,
    pub account_sequence_available: u32,
    pub account_sequence_next: u32,
    pub applied: bool,
    pub broadcast: bool,
    pub kept: bool,
    pub queued: bool,
    pub open_ledger_cost: String,
    pub validated_ledger_index: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_submit_response() {
        let json = r#"
{
    "accepted" : true,
    "account_sequence_available" : 362,
    "account_sequence_next" : 362,
    "applied" : true,
    "broadcast" : true,
    "engine_result": "tesSUCCESS",
    "engine_result_code": 0,
    "engine_result_message": "The transaction was applied. Only final in a validated ledger.",
    "kept" : true,
    "open_ledger_cost": "10",
    "queued" : false,
    "tx_blob": "1200002280000000240000016861D4838D7EA4C6800000000000000000000000000055534400000000004B4E9C06F24296074F7BC48F92A97916C6DC5EA9684000000000002710732103AB40A0490F9B7ED8DF29D246BF2D6269820A0EE7742ACDD457BEA7C7D0931EDB7446304402200E5C2DD81FDF0BE9AB2A8D797885ED49E804DBF28E806604D878756410CA98B102203349581946B0DDA06B36B35DBC20EDA27552C1F167BCF5C6ECFF49C6A46F858081144B4E9C06F24296074F7BC48F92A97916C6DC5EA983143E9D4A2B8AA0780F682D136F7A56D6724EF53754",
    "tx_json": {
      "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
      "Amount": {
        "currency": "USD",
        "issuer": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
        "value": "1"
      },
      "Destination": "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
      "Fee": "10000",
      "Flags": 2147483648,
      "Sequence": 360,
      "SigningPubKey": "03AB40A0490F9B7ED8DF29D246BF2D6269820A0EE7742ACDD457BEA7C7D0931EDB",
      "TransactionType": "Payment",
      "TxnSignature": "304402200E5C2DD81FDF0BE9AB2A8D797885ED49E804DBF28E806604D878756410CA98B102203349581946B0DDA06B36B35DBC20EDA27552C1F167BCF5C6ECFF49C6A46F8580",
      "hash": "4D5D90890F8D49519E4151938601EF3D0B30B16CD6A519D9C99102C9FA77F7E0"
    },
    "validated_ledger_index" : 21184416
}
"#;

        let _submit_response: SubmitResponse = serde_json::from_str(json).unwrap();
    }
}
