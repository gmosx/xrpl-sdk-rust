use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LedgerClosedEvent {
    pub fee_base: u32,
    pub fee_ref: u32,
    pub ledger_hash: String,
    pub ledger_index: u32,
    pub ledger_time: u64,
    pub reserve_base: u32,
    pub reserve_inc: u32,
    pub txn_count: u32,
    pub validated_ledgers: String,
}
