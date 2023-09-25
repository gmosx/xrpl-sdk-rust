use crate::{Meta, ReturnLedgerSpec, Transaction, TransactionResult};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TransactionEvent {
    pub engine_result: TransactionResult,
    pub engine_result_code: i32,
    pub engine_result_message: String,
    pub transaction: Transaction,
    pub meta: Meta,
    #[serde(flatten)]
    pub ledger_spec: ReturnLedgerSpec,
}
