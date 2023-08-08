use crate::serialize::{Serialize, Serializer};
use crate::{AccountId, Amount, IssuedAmount, TransactionCommon, TransactionType, UInt32};
use enumflags2::{bitflags, BitFlags};

/// A `TrustSet` transaction <https://xrpl.org/trustset.html>
#[derive(Debug, Clone)]
pub struct TrustSetTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<TrustSetFlags>,
    pub limit_amount: IssuedAmount,
    pub quality_in: Option<UInt32>,
    pub quality_out: Option<UInt32>,
}

impl TrustSetTransaction {
    pub fn new(account_id: AccountId, limit_amount: IssuedAmount) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            limit_amount,
            quality_in: None,
            quality_out: None,
        }
    }
}

/// `TrustSet` flags <https://xrpl.org/trustset.html#trustset-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum TrustSetFlags {
    FullyCanonicalSig = 0x80000000,
    SetfAuth = 0x00010000,
    SetNoRipple = 0x00020000,
    ClearNoRipple = 0x00040000,
    SetFreeze = 0x00100000,
    ClearFreeze = 0x00200000,
}

impl Serialize for TrustSetTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::TrustSet as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_amount("LimitAmount", Amount::Issued(self.limit_amount))?;
        if let Some(quality_in) = self.quality_in {
            s.serialize_uint32("QualityIn", quality_in)?;
        }
        if let Some(quality_out) = self.quality_out {
            s.serialize_uint32("QualityOut", quality_out)?;
        }
        Ok(())
    }
}
