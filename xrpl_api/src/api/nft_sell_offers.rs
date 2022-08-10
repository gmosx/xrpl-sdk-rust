//! The nft_sell_offers method returns a list of sell offers for a given NFToken
//! object.
//!
//! - https://xrpl.org/nft_sell_offers.html

use crate::Request;
use serde::{Deserialize, Serialize};
use xrpl_types::Amount;

#[derive(Default, Clone, Serialize)]
pub struct NftSellOffersRequest {
    /// The unique identifier of a NFToken object.
    nft_id: String,
    /// A 20-byte hex string for the ledger version to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_hash: Option<String>,
    /// The ledger index of the ledger to use, or a shortcut string to choose a
    /// ledger automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ledger_index: Option<String>,
    /// Limit the number of NFT buy offers to retrieve. This value cannot be
    /// lower than 50 or more than 500. The default is 250.
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    /// Value from a previous paginated response. Resume retrieving data where
    /// that response left off.
    #[serde(skip_serializing_if = "Option::is_none")]
    marker: Option<String>,
}

impl Request for NftSellOffersRequest {
    type Response = NftSellOffersResponse;

    fn method(&self) -> String {
        "nft_sell_offers".to_owned()
    }
}

impl NftSellOffersRequest {
    pub fn new(nft_id: &str) -> Self {
        Self {
            nft_id: nft_id.to_owned(),
            ..Default::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NFTokenOffer {
    /// The amount offered to buy the NFT for, as a String representing an amount
    /// in drops of XRP, or an object representing an amount of a fungible token.
    pub amount: Amount,
    /// A set of bit-flags for this offer. See NFTokenOffer flags for possible
    /// values.
    pub flags: u32,
    /// The ledger object ID of this offer.
    pub nft_offer_index: String,
    /// The account that placed this offer.
    pub owner: String,
}

#[derive(Debug, Deserialize)]
pub struct NftSellOffersResponse {
    /// The NFToken these offers are for, as specified in the request.
    pub nft_id: String,
    /// A list of buy offers for the token. Each of these is formatted as a Buy
    /// Offer (see below).
    pub offers: Vec<NFTokenOffer>,
    /// The limit, as specified in the request.
    pub limit: Option<u32>,
    /// Server-defined value indicating the response is paginated. Pass this to
    /// the next call to resume where this call left off. Omitted when there are
    /// no pages of information after this one.
    pub marker: Option<String>,
}
