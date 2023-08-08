use crate::serialize::{Serialize, Serializer};
use crate::{AccountId, Amount, TransactionCommon, TransactionType, UInt32};
use enumflags2::{bitflags, BitFlags};

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone)]
pub struct OfferCreateTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<OfferCreateFlags>,
    pub expiration: Option<UInt32>,
    pub offer_sequence: Option<UInt32>,
    pub taker_gets: Amount,
    pub taker_pays: Amount,
}

impl OfferCreateTransaction {
    pub fn new(account_id: AccountId, taker_gets: Amount, taker_pays: Amount) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            expiration: None,
            offer_sequence: None,
            taker_gets,
            taker_pays,
        }
    }
}

/// `OfferCreate` flags <https://xrpl.org/offercreate.html#offercreate-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OfferCreateFlags {
    FullyCanonicalSig = 0x80000000,
    Passive = 0x00010000,
    ImmediateOrCancel = 0x00020000,
    FillOrKill = 0x00040000,
    Sell = 0x00080000,
}

impl Serialize for OfferCreateTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::OfferCreate as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        if let Some(expiration) = self.expiration {
            s.serialize_uint32("Expiration", expiration)?;
        }
        if let Some(offer_sequence) = self.offer_sequence {
            s.serialize_uint32("OfferSequence", offer_sequence)?;
        }
        s.serialize_amount("TakerPays", self.taker_pays)?;
        s.serialize_amount("TakerGets", self.taker_gets)?;
        Ok(())
    }
}
