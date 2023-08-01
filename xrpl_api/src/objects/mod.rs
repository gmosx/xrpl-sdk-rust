mod account_root;
mod offer;
mod ripple_state;

pub use account_root::*;
pub use offer::*;
pub use ripple_state::*;
use serde::Deserialize;

/// Any ledger object. See <https://xrpl.org/ledger-object-types.html>
#[derive(Debug, Clone, Deserialize)]
#[serde(tag = "LedgerEntryType")]
pub enum LedgerObject {
    RippleState(RippleState),
    Offer(Offer),
    AccountRoot(AccountRoot),
    // TODO: add the rest of the entry types and remove Other variant
    #[serde(other)]
    Other,
}
