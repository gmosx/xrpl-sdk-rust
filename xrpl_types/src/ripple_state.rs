use serde::Deserialize;

use crate::{Amount, IssuedTokenAmount};

/// An ripple state in the ledger.
///
/// <https://xrpl.org/ripplestate.html>
///
/// {
///     "Balance": {
///         "currency": "USD",
///         "issuer": "rrrrrrrrrrrrrrrrrrrrBZbvji",
///         "value": "-10"
///     },
///     "Flags": 393216,
///     "HighLimit": {
///         "currency": "USD",
///         "issuer": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
///         "value": "110"
///     },
///     "HighNode": "0000000000000000",
///     "LedgerEntryType": "RippleState",
///     "LowLimit": {
///         "currency": "USD",
///         "issuer": "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
///         "value": "0"
///     },
///     "LowNode": "0000000000000000",
///     "PreviousTxnID": "E3FE6EA3D48F0C2B639448020EA4F03D4F4F8FFDB243A852A0F59177921B4879",
///     "PreviousTxnLgrSeq": 14090896,
///     "index": "9CA88CDEDFF9252B3DE183CE35B038F57282BC9503CDFA1923EF9A95DF0D6F7B"
/// }
#[derive(Debug, Clone, Deserialize)]
pub struct RippleState {
    #[serde(rename = "Balance")]
    pub balance: IssuedTokenAmount,

    #[serde(rename = "Flags")]
    pub flags: u32,

    #[serde(rename = "HighLimit")]
    pub high_limit: IssuedTokenAmount,

    #[serde(rename = "HighNode")]
    pub high_node: String,

    #[serde(rename = "HighQualityIn")]
    pub high_quality_in: Option<u32>,

    #[serde(rename = "HighQualityOut")]
    pub high_quality_out: Option<u32>,

    #[serde(rename = "LowLimit")]
    pub low_limit: IssuedTokenAmount,

    #[serde(rename = "LowNode")]
    pub low_node: String,

    #[serde(rename = "LowQualityIn")]
    pub low_quality_in: Option<u32>,

    #[serde(rename = "LowQualityOut")]
    pub low_quality_out: Option<u32>,

    pub index: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_ripple_state() {
        let json = r#"
{
    "Balance": {
        "currency": "USD",
        "issuer": "rrrrrrrrrrrrrrrrrrrrBZbvji",
        "value": "-10"
    },
    "Flags": 393216,
    "HighLimit": {
        "currency": "USD",
        "issuer": "rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn",
        "value": "110"
    },
    "HighNode": "0000000000000000",
    "LedgerEntryType": "RippleState",
    "LowLimit": {
        "currency": "USD",
        "issuer": "rsA2LpzuawewSBQXkiju3YQTMzW13pAAdW",
        "value": "0"
    },
    "LowNode": "0000000000000000",
    "PreviousTxnID": "E3FE6EA3D48F0C2B639448020EA4F03D4F4F8FFDB243A852A0F59177921B4879",
    "PreviousTxnLgrSeq": 14090896,
    "index": "9CA88CDEDFF9252B3DE183CE35B038F57282BC9503CDFA1923EF9A95DF0D6F7B"
}
"#;

        let _ripple_state: RippleState = serde_json::from_str(json).unwrap();
    }
}
