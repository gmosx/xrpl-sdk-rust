use crate::{Meta, ReturnLedgerSpec, Transaction};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionEvent {
    pub engine_result: String,
    pub transaction: Transaction,
    pub meta: Meta,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
