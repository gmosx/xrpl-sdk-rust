use serde::Deserialize;
use xrpl_types::{Amount, TransactionType};

pub trait Request {
    type Response;

    fn method(&self) -> String;
}

// #[derive(Debug, Deserialize)]
// pub struct CreatedNode {
//     #[serde(rename = "LedgerEntryType")]
//     pub ledger_entry_type: String,
//     #[serde(rename = "LedgerIndex")]
//     pub ledger_index: String,
// }

// "Account": String(
//     "rBQ9UdxF5qvfGEBEnLDkdibYHYD3yDaNFf",
// ),
// "BookDirectory": String(
//     "E5C94F1371961189FB277B38B4FB0AA0423970BB4A3C75995A04F20441870000",
// ),
// "Flags": Number(
//     131072,
// ),
// "Sequence": Number(
//     789,
// ),
// "TakerGets": Object({
//     "currency": String(
//         "ELS",
//     ),
//     "issuer": String(
//         "rHXuEaRYnnJHbDeuBH5w8yPh5uwNVh5zAg",
//     ),
//     "value": String(
//         "1000",
//     ),
// }),
// "TakerPays": String(
//     "139200000",
// ),

// #[derive(Debug, Deserialize)]
// pub struct NewFields {}

#[derive(Debug, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct Meta {
    #[serde(rename = "AffectedNodes")]
    pub affected_nodes: Vec<AffectedNode>,
    #[serde(rename = "TransactionIndex")]
    pub transaction_index: u32,
    #[serde(rename = "TransactionResult")]
    pub transaction_result: String,
}

// #[derive(Debug, Deserialize)]
// pub struct Memo {
//     #[serde(rename = "MemoData")]
//     pub memo_data: Option<String>,
// }

// TODO: rename to `Tx`? nah...
#[derive(Debug, Deserialize)]
pub struct Transaction {
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "Fee")]
    pub fee: String,

    #[serde(rename = "Destination")]
    pub destination: Option<String>,

    #[serde(rename = "Amount")]
    pub amount: Option<Amount>,

    #[serde(rename = "Flags")]
    pub flags: Option<u32>,

    #[serde(rename = "Memos")]
    // pub memos: Option<Vec<Memo>>,
    pub memos: Option<Vec<serde_json::Value>>,

    #[serde(rename = "Sequence")]
    pub sequence: u32,

    #[serde(rename = "TakerGets")]
    pub taker_gets: Option<Amount>,

    #[serde(rename = "TakerPays")]
    pub taker_pays: Option<Amount>,

    #[serde(rename = "TransactionType")]
    pub transaction_type: TransactionType,

    #[serde(rename = "TxnSignature")]
    pub txn_signature: Option<String>,

    pub date: Option<u32>, // TODO: what is the correct type?

    pub hash: String,

    pub ledger_index: Option<u32>,
}

// #TODO add Marker (https://xrpl.org/markers-and-pagination.html)
