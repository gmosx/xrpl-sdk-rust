use crate::Currency;
use serde::Serialize;

/// A book on the ledger.
#[derive(Default, Debug, Clone, Serialize)]
pub struct Book {
    /// Specification of which currency the account taking the Offer would pay.
    pub taker_gets: Currency,
    /// Specification of which currency the account taking the Offer would receive.
    pub taker_pays: Currency,
    /// Unique account address to use as a perspective for viewing offers, in the XRP Ledger's base58 format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taker: Option<String>,
    /// If true, return the current state of the order book once when you subscribe before sending updates.
    /// The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub snapshot: Option<bool>,
    /// If true, return both sides of the order book. The default is false.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub both: Option<bool>,
}

impl Book {
    pub fn new(taker_gets: Currency, taker_pays: Currency) -> Self {
        Self {
            taker_gets,
            taker_pays,
            taker: None,
            snapshot: None,
            both: None,
        }
    }

    pub fn snapshot(self, snapshot: bool) -> Self {
        Self {
            snapshot: Some(snapshot),
            ..self
        }
    }

    pub fn taker(self, taker: impl Into<String>) -> Self {
        Self {
            taker: Some(taker.into()),
            ..self
        }
    }

    pub fn both(self, both: bool) -> Self {
        Self {
            both: Some(both),
            ..self
        }
    }
}
