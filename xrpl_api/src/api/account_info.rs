use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Serialize)]
pub struct AccountInfoRequestPayload {
    pub account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub queue: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_lists: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountData {
    // TODO!
    #[serde(rename = "Account")]
    pub account: String,

    #[serde(rename = "Balance")]
    pub balance: String,

    #[serde(rename = "Sequence")]
    pub sequence: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AccountInfoResponsePayload {
    // #TODO add missing fields!
    pub account_data: AccountData,
}
