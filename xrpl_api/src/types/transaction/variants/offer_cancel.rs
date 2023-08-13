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

impl OfferCancelTransaction {
    pub fn new(account: String, offer_sequence: u32) -> Self {
        Self {
            common: TransactionCommon {
                account,
                ..Default::default()
            },
            offer_sequence,
        }
    }
}
