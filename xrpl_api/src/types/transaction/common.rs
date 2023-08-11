use crate::Meta;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct TransactionCommon {
    pub account: String,
    pub fee: String,
    pub sequence: u32,
    #[serde(rename = "AccountTxnID")]
    pub account_txn_id: Option<String>,
    pub flags: Option<u32>,
    pub last_ledger_sequence: Option<u32>,
    // pub memos: Option<Vec<Memo>>,
    pub memos: Option<Vec<serde_json::Value>>,
    pub network_id: Option<u32>,
    pub source_tag: Option<u32>,
    pub signing_pub_key: Option<String>,
    pub ticket_sequence: Option<u32>,
    pub txn_signature: Option<String>,

    /// Close time of the ledger in which the transaction is included
    #[serde(rename = "date")]
    pub date: Option<u64>,

    /// Transaction hash
    #[serde(rename = "hash")]
    pub hash: String,

    /// The ledger index of the ledger that includes this transaction.
    #[serde(rename = "ledger_index")]
    pub ledger_index: Option<u32>,
    /// If true, this data comes from a validated ledger version; if omitted or
    /// set to false, this data is not final.
    #[serde(rename = "validated")]
    pub validated: Option<bool>,

    /// Meta is present in transactions returned by https://xrpl.org/ledger.html and
    /// also <https://xrpl.org/tx.html>. In other API
    /// methods it is found outside (next to) the transaction field.
    #[serde(rename = "meta", alias = "metaData")]
    pub meta: Option<Meta>,

    /// `owner_funds` is present in transactions returned by book subscription, see
    /// <https://xrpl.org/subscribe.html#order-book-streams>.
    #[serde(rename = "owner_funds")]
    pub owner_funds: Option<String>,
}

// #[derive(Debug, Deserialize)]
// pub struct Memo {
//     #[serde(rename = "MemoData")]
//     pub memo_data: Option<String>,
// }
