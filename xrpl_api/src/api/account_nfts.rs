//! The account_nfts method returns a list of NFToken objects for the specified
//! account.
//!
//! <https://xrpl.org/account_nfts.html>

use crate::Request;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize)]
pub struct AccountNftsRequest {
    /// The unique identifier of an account, typically the account's Address.
    /// The request returns a list of NFTs owned by this account.
    account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ledger_index: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// Value from a previous paginated response. Resume retrieving data where
    /// that response left off.
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
}

impl Request for AccountNftsRequest {
    type Response = AccountNftsResponse;

    fn method(&self) -> String {
        "account_nfts".to_owned()
    }
}

impl AccountNftsRequest {
    pub fn new(account: &str) -> Self {
        Self {
            account: account.to_owned(),
            ..Default::default()
        }
    }
}

// TODO: consider extracting as a type.

/// The NFToken object represents a single non-fungible token (NFT). It is not
/// stored on its own, but is contained in a NFTokenPage object alongside other
/// NFTs.
#[derive(Debug, Serialize, Deserialize)]
pub struct NFToken {
    /// A bit-map of boolean flags enabled for this NFToken.
    #[serde(rename = "Flags")]
    pub flags: u32,
    /// The account that issued this NFToken.
    #[serde(rename = "Issuer")]
    pub issuer: String,
    /// The unique identifier of this NFToken, in hexadecimal.
    #[serde(rename = "NFTokenID")]
    pub nftoken_id: String,
    /// The unscrambled version of this token's taxon. Several tokens with the
    /// same taxon might represent instances of a limited series.
    #[serde(rename = "NFTokenTaxon")]
    pub nftoken_taxon: u32,
    /// The URI data associated with this NFToken, in hexadecimal.
    #[serde(rename = "URI")]
    pub uri: String,
    /// The token sequence number of this NFToken, which is unique for its issuer.
    pub nft_serial: u32,
}

#[derive(Debug, Deserialize)]
pub struct AccountNftsResponse {
    pub account: String,
    pub account_nfts: Vec<NFToken>,
    pub validated: bool,
    // #TODO add missing fields
}
