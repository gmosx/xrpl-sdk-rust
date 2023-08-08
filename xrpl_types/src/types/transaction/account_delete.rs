use crate::serialize::{Serialize, Serializer};
use crate::{AccountId, TransactionCommon, TransactionType, UInt32};
use enumflags2::{bitflags, BitFlags};

/// An `AccountDelete` transaction <https://xrpl.org/accountdelete.html>
#[derive(Debug, Clone)]
pub struct AccountDeleteTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<AccountDeleteFlags>,
    pub destination: AccountId,
    pub destination_tag: Option<UInt32>,
}

impl AccountDeleteTransaction {
    pub fn new(account_id: AccountId, destination: AccountId) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            destination,
            destination_tag: None,
        }
    }
}

#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum AccountDeleteFlags {
    FullyCanonicalSig = 0x80000000,
}

impl Serialize for AccountDeleteTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16("TransactionType", TransactionType::AccountDelete as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_account_id("Destination", self.destination)?;
        if let Some(destination_tag) = self.destination_tag {
            s.serialize_uint32("DestinationTag", destination_tag)?;
        }
        Ok(())
    }
}
