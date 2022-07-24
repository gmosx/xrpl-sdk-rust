use crate::{client::RpcRequest, Client, Result};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use xrpl_types::{Amount, TransactionType};

#[derive(Default, Clone, Serialize)]
pub struct AccountTxParams {
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index_min: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index_max: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    // TODO: add more parameters!
}

/// - https://xrpl.org/account_tx.html
#[must_use = "Does nothing until you send or execute it"]
#[derive(Default, Clone)]
pub struct AccountTxRequest {
    client: Client,
    params: AccountTxParams,
}

impl AccountTxRequest {
    // TODO: add more builders.
    pub fn limit(self, limit: u32) -> Self {
        Self {
            params: AccountTxParams {
                limit: Some(limit),
                ..self.params
            },
            ..self
        }
    }

    pub async fn execute<T: DeserializeOwned>(self) -> Result<T> {
        let request = RpcRequest {
            method: "account_tx".to_string(),
            params: vec![self.params],
        };
        self.client.send_old::<AccountTxParams, T>(request).await
    }

    pub async fn send(self) -> Result<AccountTxResponse> {
        self.execute().await
    }
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

    pub date: u32, // TODO: what is the correct type?

    pub hash: String,

    pub ledger_index: u32,
}

#[derive(Debug, Deserialize)]
pub struct AccountTransaction {
    pub meta: Meta,
    // pub tx: serde_json::Value,
    pub tx: Transaction,
    pub validated: bool,
}

#[derive(Debug, Deserialize)]
pub struct AccountTxResponse {
    pub account: String,
    pub limit: u32,
    pub transactions: Vec<AccountTransaction>,
}

impl Client {
    pub fn account_tx(&self, account: &str) -> AccountTxRequest {
        AccountTxRequest {
            client: self.clone(),
            params: AccountTxParams {
                account: account.to_string(),
                ledger_hash: None,
                ledger_index: None,
                ledger_index_min: None,
                ledger_index_max: None,
                limit: None,
            },
        }
    }
}
