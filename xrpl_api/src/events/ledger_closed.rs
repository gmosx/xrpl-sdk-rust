use serde::Deserialize;
use xrpl_types::LedgerTimestamp;

#[derive(Debug, Deserialize)]
pub struct LedgerClosedEvent {
    pub fee_base: u32,
    pub fee_ref: Option<u32>,
    pub ledger_hash: String,
    pub ledger_index: u32,
    pub ledger_time: LedgerTimestamp,
    pub reserve_base: u32,
    pub reserve_inc: u32,
    pub txn_count: u32,
    pub validated_ledgers: String,
}
