use enumflags2::{bitflags, BitFlags};
use serde::Deserialize;

use crate::IssuedAmount;

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
#[serde(rename_all = "PascalCase")]
pub struct RippleState {
    pub balance: IssuedAmount,
    pub flags: BitFlags<RippleStateFlags>,
    pub high_limit: IssuedAmount,
    /// This field is mandatory on `RippleState` object, but we leave it optional, such
    /// that we can parse the object from `CreateNode` fields where it is not set. See <https://xrpl.org/transaction-metadata.html>
    pub high_node: Option<String>,
    pub high_quality_in: Option<u32>,
    pub high_quality_out: Option<u32>,
    pub low_limit: IssuedAmount,
    /// This field is mandatory on `RippleState` object, but we leave it optional, such
    /// that we can parse the object from `CreateNode` fields where it is not set. See <https://xrpl.org/transaction-metadata.html>
    pub low_node: Option<String>,
    pub low_quality_in: Option<u32>,
    pub low_quality_out: Option<u32>,
    pub index: Option<String>,
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RippleStateFlags {
    LowReserve = 0x00010000,
    HighReserve = 0x00020000,
    LowAuth = 0x00040000,
    HighAuth = 0x00080000,
    LowNoRipple = 0x00100000,
    HighNoRipple = 0x00200000,
    LowFreeze = 0x00400000,
    HighFreeze = 0x00800000,
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
