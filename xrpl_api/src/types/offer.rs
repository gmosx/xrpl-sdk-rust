use enumflags2::{bitflags, BitFlags};
use serde::{Deserialize, Serialize};
use xrpl_types::Amount;

/// The object was placed as a passive offer
pub const LSF_PASSIVE: u32 = 0x00010000;

/// The object was placed as a sell offer
pub const LSF_SELL: u32 = 0x00020000;

/// An offer in the ledger.
///
/// <https://xrpl.org/offer.html>
///
/// {
///     "Account": "rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt",
///     "BookDirectory": "ACC27DE91DBA86FC509069EAF4BC511D73128B780F2E54BF5E07A369E2446000",
///     "BookNode": "0000000000000000",
///     "Flags": 131072,
///     "LedgerEntryType": "Offer",
///     "OwnerNode": "0000000000000000",
///     "PreviousTxnID": "F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF",
///     "PreviousTxnLgrSeq": 14524914,
///     "Sequence": 866,
///     "TakerGets": {
///         "currency": "XAG",
///         "issuer": "r9Dr5xwkeLegBeXq6ujinjSBLQzQ1zQGjH",
///         "value": "37"
///     },
///     "TakerPays": "79550000000",
///     "index": "96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797"
/// }
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Offer {
    pub account: String,
    pub book_directory: String,
    pub book_node: Option<String>,
    pub expiration: Option<u32>,
    pub flags: BitFlags<OfferFlags>,
    pub owner_node: String,
    pub sequence: u32,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
    /// Declared optional since it is not part of transaction metadata fields <https://xrpl.org/transaction-metadata.html#modifiednode-fields>
    #[serde(rename = "PreviousTxnID")]
    pub previous_txn_id: Option<String>,
    /// Declared optional since it is not part of transaction metadata fields <https://xrpl.org/transaction-metadata.html#modifiednode-fields>
    pub previous_txn_lgr_seq: Option<u32>,
    #[serde(rename = "index")]
    pub index: Option<String>,

    /// `owner_funds` is present in offers returned by `book_offers` method, see
    /// <https://xrpl.org/book_offers.html#response-format>.
    #[serde(rename = "owner_funds")]
    pub owner_funds: Option<String>,
    /// `taker_gets_funded` may be present in offers returned by `book_offers` method, see
    /// <https://xrpl.org/book_offers.html#response-format>.
    #[serde(rename = "taker_gets_funded")]
    pub taker_gets_funded: Option<Amount>,
    /// taker_pays_funded may be present in offers returned by `book_offers` method, see
    /// <https://xrpl.org/book_offers.html#response-format>.
    #[serde(rename = "taker_pays_funded")]
    pub taker_pays_funded: Option<Amount>,
    /// `quality` is present in offers returned by `book_offers` method, see
    /// <https://xrpl.org/book_offers.html#response-format>.
    #[serde(rename = "quality")]
    pub quality: Option<String>,
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OfferFlags {
    Passive = 0x00010000,
    Sell = 0x00020000,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deserialize_offer() {
        let json = r#"
{
    "Account": "rBqb89MRQJnMPq8wTwEbtz4kvxrEDfcYvt",
    "BookDirectory": "ACC27DE91DBA86FC509069EAF4BC511D73128B780F2E54BF5E07A369E2446000",
    "BookNode": "0000000000000000",
    "Flags": 131072,
    "LedgerEntryType": "Offer",
    "OwnerNode": "0000000000000000",
    "PreviousTxnID": "F0AB71E777B2DA54B86231E19B82554EF1F8211F92ECA473121C655BFC5329BF",
    "PreviousTxnLgrSeq": 14524914,
    "Sequence": 866,
    "TakerGets": {
        "currency": "XAG",
        "issuer": "r9Dr5xwkeLegBeXq6ujinjSBLQzQ1zQGjH",
        "value": "37"
    },
    "TakerPays": "79550000000",
    "index": "96F76F27D8A327FC48753167EC04A46AA0E382E6F57F32FD12274144D00F1797"
}
"#;

        let _offer: Offer = serde_json::from_str(json).unwrap();
    }
}
