use crate::serialize::{Serialize, Serializer};
use crate::{AccountId, TransactionCommon, TransactionType, UInt32};
use enumflags2::{bitflags, BitFlags};

/// An `OfferCancel` transaction <https://xrpl.org/offercancel.html>
#[derive(Debug, Clone)]
pub struct OfferCancelTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<OfferCancelFlags>,
    pub offer_sequence: UInt32,
}

impl OfferCancelTransaction {
    pub fn new(account_id: AccountId, offer_sequence: UInt32) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            offer_sequence,
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OfferCancelFlags {
    FullyCanonicalSig = 0x80000000,
}

impl Serialize for OfferCancelTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::OfferCancel as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_uint32("OfferSequence", self.offer_sequence)?;
        Ok(())
    }
}
