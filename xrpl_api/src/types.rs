use serde::Deserialize;

// Submodules defining ledger objects: (https://xrpl.org/ledger-object-types.html)

mod account_root;
mod meta;
mod nf_token_offer;
mod offer;
mod ripple_state;
mod transaction;
mod transactions;

pub use account_root::*;
pub use meta::*;
pub use nf_token_offer::*;
pub use offer::*;
pub use ripple_state::*;
pub use transaction::*;
pub use transactions::*;

pub trait Request {
    type Response;

    fn method(&self) -> String;
}

/// Ledger object. See <https://xrpl.org/ledger-object-types.html>
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "LedgerEntryType")]
pub enum LedgerObject {
    AccountRoot(AccountRoot),
    // TODO add model for remaining obejcts
    Amendments,
    Check,
    DepositPreauth,
    DirectoryNode,
    Escrow,
    FeeSettings,
    LedgerHashes,
    NegativeUNL,
    NFTokenOffer,
    NFTokenPage,
    Offer(Offer),
    PayChannel,
    RippleState(RippleState),
    SignerList,
    Ticket,
}
