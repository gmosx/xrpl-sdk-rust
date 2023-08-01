use crate::Amount;
use serde::{Deserialize, Serialize};

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
