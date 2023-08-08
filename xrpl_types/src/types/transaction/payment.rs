use crate::serialize::{Serialize, Serializer};
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
        s.serialize_uint16("TransactionType", TransactionType::Payment as u16)?;
        self.common.serialize(s)?;
        s.serialize_uint32("Flags", self.flags.bits())?;
        s.serialize_amount("Amount", self.amount)?;
        s.serialize_account_id("Destination", self.destination)?;
        if let Some(destination_tag) = self.destination_tag {
            s.serialize_uint32("DestinationTag", destination_tag)?;
        }
        if let Some(invoice_id) = self.invoice_id {
            s.serialize_hash256("InvoiceID", invoice_id)?;
        }
        if let Some(send_max) = self.send_max {
            s.serialize_amount("SendMax", send_max)?;
        }
        if let Some(deliver_min) = self.deliver_min {
            s.serialize_amount("DeliverMin", deliver_min)?;
        }
        Ok(())
    }
}
