use crate::TransactionCommon;
use serde::{Deserialize, Serialize};

/// An `AccountSet` transaction <https://xrpl.org/accountset.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountSetTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub clear_flag: Option<u32>,
    pub domain: Option<String>,
    pub email_hash: Option<String>,
    pub message_key: Option<String>,
    #[serde(rename = "NFTokenMinter")]
    pub nf_token_minter: Option<String>,
    pub set_flag: Option<u32>,
    pub transfer_rate: Option<u32>,
    pub tick_size: Option<u8>,
    pub wallet_locator: Option<String>,
    pub wallet_size: Option<u32>,
}
