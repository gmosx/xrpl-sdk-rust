use crate::{Meta, ReturnLedgerSpec, Transaction, TransactionResult};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionEvent {
    pub engine_result: TransactionResult,
    pub engine_result_code: i32,
    pub engine_result_message: String,
    pub transaction: Transaction,
    pub meta: Meta,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}

// BASELINE: test that TransactionEvent fails when transaction has unknown type — see issue #41
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_transaction_event_fails_on_unknown_transaction_type() {
        // This is the exact path described in issue #41 — subscribing to
        // streams="transactions" and receiving an OracleSet event.
        // Currently FAILS — TransactionEvent cannot deserialize unknown TransactionType.
        let json = r#"{
            "engine_result": "tesSUCCESS",
            "engine_result_code": 0,
            "engine_result_message": "The transaction was applied.",
            "meta": {
                "AffectedNodes": [],
                "TransactionIndex": 0,
                "TransactionResult": "tesSUCCESS"
            },
            "transaction": {
                "TransactionType": "OracleSet",
                "Account": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
                "Fee": "12",
                "Sequence": 1
            },
            "ledger_hash": "abc123",
            "ledger_index": 1000,
            "validated": true
        }"#;
        // After fix: should deserialize successfully
        let _: TransactionEvent = serde_json::from_str(json).unwrap();
    }
}
