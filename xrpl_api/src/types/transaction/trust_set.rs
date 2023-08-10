use crate::TransactionCommon;
use serde::Deserialize;
use xrpl_types::IssuedAmount;

/// A `TrustSet` transaction <https://xrpl.org/trustset.html>
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TrustSetTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub limit_amount: IssuedAmount,
    pub quality_in: Option<u32>,
    pub quality_out: Option<u32>,
}
