use crate::{Amount, TransactionCommon};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::PaymentFlags;

/// An `Payment` transaction <https://xrpl.org/payment.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct PaymentTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<PaymentFlags>,
    pub amount: Amount,
    pub destination: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_tag: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_max: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliver_min: Option<Amount>,
}

#[cfg(test)]
mod test {
    use crate::PaymentTransaction;

    #[test]
    fn test_payment_deserialize() {
        let json = r#"
{
  "TransactionType" : "Payment",
  "Account" : "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
  "Destination" : "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
  "Amount" : {
     "currency" : "USD",
     "value" : "1",
     "issuer" : "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn"
  },
  "Fee": "12",
  "Flags": 2147483648,
  "Sequence": 2
}
        "#;

        let _: PaymentTransaction = serde_json::from_str(json).unwrap();
    }
}
