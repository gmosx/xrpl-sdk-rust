use crate::{Amount, TransactionCommon};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::{LedgerTimestamp, OfferCreateFlags};

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct OfferCreateTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<OfferCreateFlags>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration: Option<LedgerTimestamp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offer_sequence: Option<u32>,
    pub taker_gets: Amount,
    pub taker_pays: Amount,

    /// `owner_funds` is present in transactions returned by book subscription, see
    /// <https://xrpl.org/subscribe.html#order-book-streams>.
    #[serde(rename = "owner_funds")]
    pub owner_funds: Option<String>,
}

#[cfg(test)]
mod test {
    use crate::OfferCreateTransaction;

    #[test]
    fn test_offer_create_deserialize() {
        let json = r#"
{
    "TransactionType": "OfferCreate",
    "Account": "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
    "Fee": "12",
    "Flags": 0,
    "LastLedgerSequence": 7108682,
    "Sequence": 8,
    "TakerGets": "6000000",
    "TakerPays": {
      "currency": "GKO",
      "issuer": "ruazs5h1qEsqpke88pcqnaseXdm6od2xc",
      "value": "2"
    }
}
        "#;

        let _: OfferCreateTransaction = serde_json::from_str(json).unwrap();
    }
}
