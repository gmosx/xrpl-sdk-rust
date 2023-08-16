use crate::TransactionCommon;
use serde::{Deserialize, Serialize};
use xrpl_types::{Amount, LedgerTimestamp};

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCreateTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub expiration: Option<LedgerTimestamp>,
    pub offer_sequence: Option<u32>,
    pub taker_gets: Amount,
    pub taker_pays: Amount,

    /// `owner_funds` is present in transactions returned by book subscription, see
    /// <https://xrpl.org/subscribe.html#order-book-streams>.
    #[serde(rename = "owner_funds")]
    pub owner_funds: Option<String>,
}
