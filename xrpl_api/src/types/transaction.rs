use crate::{Amount, Meta};
use serde::Deserialize;
use xrpl_types::TransactionType;

// TODO: rename to `Tx`? nah...
#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "SourceTag")]
    pub source_tag: Option<u32>,

    #[serde(rename = "Fee")]
    pub fee: String,

    #[serde(rename = "Destination")]
    pub destination: Option<String>,

    #[serde(rename = "DestinationTag")]
    pub destination_tag: Option<u32>,

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

    #[serde(rename = "metaData")]
    pub meta: Option<Meta>,
}
