use crate::{Amount, Transaction, UInt32};
use enumflags2::bitflags;

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone)]
pub struct OfferCreateTransaction {
    pub common: Transaction,
    pub expiration: Option<UInt32>,
    pub offer_sequence: Option<UInt32>,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}

/// `OfferCreate` flags <https://xrpl.org/offercreate.html#offercreate-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OfferCreateFlags {
    Passive = 0x00010000,
    ImmediateOrCancel = 0x00020000,
    FillOrKill = 0x00040000,
    Sell = 0x00080000,
}
