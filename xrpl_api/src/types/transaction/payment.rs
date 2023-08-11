use crate::TransactionCommon;
use serde::{Deserialize, Serialize};
use xrpl_types::Amount;

/// An `Payment` transaction <https://xrpl.org/payment.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    pub amount: Amount,
    pub destination: String,
    pub destination_tag: Option<u32>,
    pub invoice_id: Option<String>,
    pub send_max: Option<Amount>,
    pub deliver_min: Option<Amount>,
}
