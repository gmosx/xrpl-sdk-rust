use serde::Deserialize;
use xrpl_types::Amount;

#[derive(Debug, Clone, Deserialize)]
pub enum AffectedNode {
    CreatedNode {
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "NewFields")]
        new_fields: serde_json::Value,
    },
    ModifiedNode {
        #[serde(rename = "LedgerEntryType")]
        ledger_entry_type: String,
        #[serde(rename = "LedgerIndex")]
        ledger_index: String,
        #[serde(rename = "FinalFields")]
        final_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousFields")]
        previous_fields: Option<serde_json::Value>,
        #[serde(rename = "PreviousTxnID")]
        previous_txn_id: Option<String>,
        #[serde(rename = "PreviousTxnLgrSeq")]
        previous_txn_lgr_seq: Option<u32>,
    },
    DeletedNode {
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
#[serde(rename_all = "PascalCase")]
pub struct Meta {
    pub affected_nodes: Vec<AffectedNode>,
    pub transaction_index: u32,
    pub transaction_result: String,
    #[serde(rename = "delivered_amount")]
    pub delivered_amount: Option<Amount>,
}
