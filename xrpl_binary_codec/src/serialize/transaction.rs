use crate::serialize::Serialize;
use crate::serializer::SerializerT;
use crate::BinaryCodecError;
use xrpl_types::{Amount, OfferCreateTransaction, TrustSetTransaction};
use crate::field_id::{FieldCode, FieldId};

impl Serialize for TrustSetTransaction {
    fn serialize<S: SerializerT>(&self, s: &mut S) -> Result<(), BinaryCodecError> {
        // serializer.se
        todo!()
    }
}

impl Serialize for OfferCreateTransaction {
    fn serialize<S: SerializerT>(&self, s: &mut S) -> Result<(), BinaryCodecError> {
        s.serialize_uint16(FieldId::uint16_1(FieldCode(2)), self.common.transaction_type as u16)?;
        if let Some(network_id) = self.common.network_id {
            s.serialize_uint32(FieldId::uint32_2(FieldCode(1)), network_id)?;
        }
        s.serialize_uint32(FieldId::uint32_2(FieldCode(2)), self.common.flags.bits() | self.flags.bits())?;
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
    use crate::serializer::Serializer;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;
    use enumflags2::BitFlags;
    use xrpl_types::{
        AccountId, Amount, Blob, CurrencyCode, DropsAmount, IssuedValue, OfferCreateTransaction,
        Transaction, TransactionType,
    };

    fn serializer() -> Serializer<Vec<u8>> {
        Serializer::new(Vec::new())
    }

    /// Tests the example <https://xrpl.org/serialization.html#examples>
    #[test]
    fn test_serialize_offer_create() {
        let mut s = serializer();
        let tx = OfferCreateTransaction {
            common: Transaction {
                account: AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap(),
                transaction_type: TransactionType::OfferCreate,
                fee: Some(DropsAmount::from_drops(10).unwrap()),
                sequence: Some(1752792),
                account_txn_id: None,
                flags: BitFlags::default(),
                last_ledger_sequence: None,
                network_id: None,
                source_tag: None,
                signing_pub_key: Some(Blob(hex::decode("03EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3").unwrap())),
                ticket_sequence: None,
                txn_signature: Some(Blob(hex::decode("30440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C").unwrap())),
            },
            expiration: Some(595640108),
            flags: BitFlags::from_bits(524288).unwrap(),
            offer_sequence: Some(1752791),
            taker_gets: Amount::drops(15000000000).unwrap(),
            taker_pays: Amount::issued(
                IssuedValue::from_mantissa_exponent(70728, -1).unwrap(),
                CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
                AccountId::from_address("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B").unwrap(),
            ).unwrap(),
        };

        tx.serialize(&mut s).unwrap();
        assert_eq!(hex::encode_upper(s.into_inner()), "120007220008000024001ABED82A2380BF2C2019001ABED764D55920AC9391400000000000000000000000000055534400000000000A20B3C85F482532A9578DBB3950B85CA06594D165400000037E11D60068400000000000000A732103EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3744630440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C8114DD76483FACDEE26E60D8A586BB58D09F27045C46");
    }
}
