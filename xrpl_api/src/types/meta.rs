use crate::Amount;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum AffectedNode {
    // CreateNode {},
    // CreatedNode(serde_json::Value),
    // CreatedNode(CreatedNode),
    CreatedNode {
        // TODO: more fields missing?
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "NewFields")]
        new_fields: serde_json::Value,
    },
    // ModifiedNode(serde_json::Value),
    ModifiedNode {
        // TODO: more fields missing?
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "FinalFields")]
        final_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousFields")]
        previous_fields: Option<serde_json::Value>,
    },
    // DeletedNode(serde_json::Value),
    DeletedNode {
        // TODO: more fields missing?
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "FinalFields")]
        final_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousFields")]
        previous_fields: Option<serde_json::Value>,
    },
}

#[derive(Debug, Clone, Deserialize)]
pub struct Meta {
    #[serde(rename = "AffectedNodes")]
    pub affected_nodes: Vec<AffectedNode>,
    #[serde(rename = "TransactionIndex")]
    pub transaction_index: u32,
    #[serde(rename = "TransactionResult")]
    pub transaction_result: String,
    #[serde(rename = "delivered_amount")]
    pub delivered_amount: Option<Amount>,
}
