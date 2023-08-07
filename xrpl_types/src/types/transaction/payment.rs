use crate::serialize::{FieldCode, Serialize, Serializer};
use crate::{AccountId, Amount, Hash256, TransactionCommon, TransactionType, UInt32};
use enumflags2::{bitflags, BitFlags};

/// An `Payment` transaction <https://xrpl.org/payment.html>
#[derive(Debug, Clone)]
pub struct PaymentTransaction {
    pub common: TransactionCommon,
    pub flags: BitFlags<PaymentFlags>,
    pub amount: Amount,
    pub destination: AccountId,
    pub destination_tag: Option<UInt32>,
    pub invoice_id: Option<Hash256>,
    pub send_max: Option<Amount>,
    pub deliver_min: Option<Amount>,
}

impl PaymentTransaction {
    pub fn new(account_id: AccountId, amount: Amount, destination: AccountId) -> Self {
        Self {
            common: TransactionCommon::new(account_id),
            flags: Default::default(),
            amount,
            destination,
            destination_tag: None,
            invoice_id: None,
            send_max: None,
            deliver_min: None,
        }
    }
}

/// `Payment` flags <https://xrpl.org/payment.html#payment-flags>
#[bitflags]
#[repr(u32)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum PaymentFlags {
    FullyCanonicalSig = 0x80000000,
    NoDirectRipple = 0x00010000,
    PartialPayment = 0x00020000,
    LimitQuality = 0x00040000,
}

impl Serialize for PaymentTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16(FieldCode(2), TransactionType::Payment as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32(FieldCode(2), self.flags.bits())?;
        s.serialize_amount(FieldCode(1), self.amount)?;
        s.serialize_account_id(FieldCode(3), self.destination)?;
        if let Some(destination_tag) = self.destination_tag {
            s.serialize_uint32(FieldCode(14), destination_tag)?;
        }
        if let Some(invoice_id) = self.invoice_id {
            s.serialize_hash256(FieldCode(17), invoice_id)?;
        }
        if let Some(send_max) = self.send_max {
            s.serialize_amount(FieldCode(9), send_max)?;
        }
        if let Some(deliver_min) = self.deliver_min {
            s.serialize_amount(FieldCode(10), deliver_min)?;
        }
        Ok(())
    }
}
