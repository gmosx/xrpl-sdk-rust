use crate::{IssuedAmount, TransactionCommon};
use enumflags2::BitFlags;
use serde::{Deserialize, Serialize};
use xrpl_types::TrustSetFlags;

/// A `TrustSet` transaction <https://xrpl.org/trustset.html>
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct TrustSetTransaction {
    #[serde(flatten)]
    pub common: TransactionCommon,
    #[serde(default)]
    pub flags: BitFlags<TrustSetFlags>,
    pub limit_amount: IssuedAmount,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_in: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality_out: Option<u32>,
}

#[cfg(test)]
mod test {
    use crate::TrustSetTransaction;

    #[test]
    fn test_trust_set_deserialize() {
        let json = r#"
{
    "TransactionType": "TrustSet",
    "Account": "ra5nK24KXen9AHvsdFTKHSANinZseWnPcX",
    "Fee": "12",
    "Flags": 262144,
    "LastLedgerSequence": 8007750,
    "LimitAmount": {
      "currency": "USD",
      "issuer": "rsP3mgGb2tcYUrxiLFiHJiQXhsziegtwBc",
      "value": "100"
    },
    "Sequence": 12
}
        "#;

        let _: TrustSetTransaction = serde_json::from_str(json).unwrap();
    }
}
