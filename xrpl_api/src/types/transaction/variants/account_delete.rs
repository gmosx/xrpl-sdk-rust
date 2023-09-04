use crate::TransactionCommon;
use serde::{Deserialize, Serialize};

/// An `AccountDelete` transaction <https://xrpl.org/accountdelete.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountDeleteTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub destination: String,
    pub destination_tag: Option<u32>,
}
