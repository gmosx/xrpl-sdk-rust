use crate::TransactionCommon;
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::OfferCancelFlags;

/// An `OfferCancel` transaction <https://xrpl.org/offercancel.html>
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCancelTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<OfferCancelFlags>,
    pub offer_sequence: u32,
}

#[cfg(test)]
mod test {
    use crate::OfferCancelTransaction;

    #[test]
    fn test_offer_cancel_deserialize() {
        let json = r#"
{
    "TransactionType": "OfferCancel",
    "Account": "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
    "Fee": "12",
    "Flags": 0,
    "LastLedgerSequence": 7108629,
    "OfferSequence": 6,
    "Sequence": 7
}
        "#;

        let _: OfferCancelTransaction = serde_json::from_str(json).unwrap();
    }
}
