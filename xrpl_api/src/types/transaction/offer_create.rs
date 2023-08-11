use crate::TransactionCommon;
use serde::{Deserialize, Serialize};
use xrpl_types::Amount;

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCreateTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub expiration: Option<u32>,
    pub offer_sequence: Option<u32>,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}
