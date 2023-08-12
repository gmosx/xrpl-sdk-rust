use crate::TransactionCommon;
use serde::{Deserialize, Serialize};

/// An `OfferCancel` transaction <https://xrpl.org/offercancel.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCancelTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub offer_sequence: u32,
}
