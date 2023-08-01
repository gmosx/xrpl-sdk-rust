use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ValidationReceivedEvent {
    pub base_fee: u32,
    pub cookie: Option<String>,
    pub flags: u32,
    pub ledger_hash: String,
    pub ledger_index: String,
    pub signature: String,
    // #TODO add missing fields
}
