use crate::serialize::{FieldCode, FieldId, Serialize, Serializer};
use crate::{Amount, Transaction, UInt32};
use enumflags2::{bitflags, BitFlags};

/// An `OfferCreate` transaction <https://xrpl.org/offercreate.html>
#[derive(Debug, Clone)]
pub struct OfferCreateTransaction {
    pub common: Transaction,
    pub flags: BitFlags<OfferCreateFlags>,
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

impl Serialize for OfferCreateTransaction {
    fn serialize<S: Serializer>(&self, s: &mut S) -> Result<(), S::Error> {
        s.serialize_uint16(
            FieldId::uint16_1(FieldCode(2)),
            self.common.transaction_type as u16,
        )?;
        if let Some(network_id) = self.common.network_id {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(1)), network_id)?;
        }
        s.serialize_uint32(
            FieldId::uint32_2(FieldCode(2)),
            self.common.flags.bits() | self.flags.bits(),
        )?;
        if let Some(source_tag) = self.common.source_tag {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(3)), source_tag)?;
        }
        if let Some(sequence) = self.common.sequence {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(4)), sequence)?;
        }
        if let Some(expiration) = self.expiration {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(10)), expiration)?;
        }
        if let Some(offer_sequence) = self.offer_sequence {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(25)), offer_sequence)?;
        }
        if let Some(last_ledger_sequence) = self.common.last_ledger_sequence {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(27)), last_ledger_sequence)?;
        }
        if let Some(ticket_sequence) = self.common.ticket_sequence {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(41)), ticket_sequence)?;
        }
        if let Some(account_txn_id) = self.common.account_txn_id {
            s.serialize_hash256(FieldId::hash256_5(FieldCode(9)), account_txn_id)?;
        }
        s.serialize_amount(FieldId::amount_6(FieldCode(4)), self.taker_pays)?;
        s.serialize_amount(FieldId::amount_6(FieldCode(5)), self.taker_gets)?;
        if let Some(fee) = self.common.fee {
            s.serialize_amount(FieldId::amount_6(FieldCode(8)), Amount::Drops(fee))?;
        }
        if let Some(signing_pub_key) = self.common.signing_pub_key.as_ref() {
            s.serialize_blob(FieldId::blob_7(FieldCode(3)), signing_pub_key)?;
        }
        if let Some(txn_signature) = self.common.txn_signature.as_ref() {
            s.serialize_blob(FieldId::blob_7(FieldCode(4)), txn_signature)?;
        }
        s.serialize_account_id(FieldId::account_id_8(FieldCode(1)), self.common.account)?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{
        AccountId, Amount, Blob, CurrencyCode, DropsAmount, IssuedValue, OfferCreateTransaction,
        Transaction, TransactionType,
    };
    use ascii::AsciiChar;
    use assert_matches::assert_matches;
    use enumflags2::BitFlags;

    fn serializer() -> Serializer<Vec<u8>> {
        Serializer::new(Vec::new())
    }
}
