use crate::error::BinaryCodecError;
use crate::field_id::{type_code, FieldCode, FieldId, TypeCode};
use std::io::Write;
use xrpl_types::Uint64;
use xrpl_types::{
    AccountId, Amount, Blob, CurrencyCode, DropsAmount, Hash128, Hash160, Hash256, IssuedValue,
    UInt16, UInt32, UInt8,
};

pub const HASH_PREFIX_TRANSACTION: [u8; 4] = [0x53, 0x4E, 0x44, 0x00];
pub const HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE: [u8; 4] = [0x53, 0x54, 0x58, 0x00];

pub struct Serializer<W> {
    writer: W,
    /// Previously serialized field id
    previous_field_id: Option<(TypeCode, FieldCode)>,
}

impl<W> Serializer<W> {
    pub fn new(writer: W) -> Self {
        Self {
            writer,
            previous_field_id: None,
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

pub trait SerializerT {
    fn serialize_account_id(
        &mut self,
        field_id: FieldId<{ type_code::ACCOUNT_ID_8 }>,
        account_id: AccountId,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_amount(
        &mut self,
        field_id: FieldId<{ type_code::AMOUNT_6 }>,
        amount: Amount,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_blob(
        &mut self,
        field_id: FieldId<{ type_code::BLOB_7 }>,
        blob: &Blob,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_hash128(
        &mut self,
        field_id: FieldId<{ type_code::HASH128_4 }>,
        hash128: Hash128,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_hash160(
        &mut self,
        field_id: FieldId<{ type_code::HASH160_17 }>,
        hash160: Hash160,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_hash256(
        &mut self,
        field_id: FieldId<{ type_code::HASH256_5 }>,
        hash256: Hash256,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_uint8(
        &mut self,
        field_id: FieldId<{ type_code::UINT8_16 }>,
        uint8: UInt8,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_uint16(
        &mut self,
        field_id: FieldId<{ type_code::UINT16_1 }>,
        uint16: UInt16,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_uint32(
        &mut self,
        field_id: FieldId<{ type_code::UINT32_2 }>,
        uint32: UInt32,
    ) -> Result<(), BinaryCodecError>;

    fn serialize_uint64(
        &mut self,
        field_id: FieldId<{ type_code::UINT64_3 }>,
        uint64: Uint64,
    ) -> Result<(), BinaryCodecError>;
}

impl<W: Write> SerializerT for Serializer<W> {
    fn serialize_account_id(
        &mut self,
        field_id: FieldId<{ type_code::ACCOUNT_ID_8 }>,
        account_id: AccountId,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_account_id(account_id)?;
        Ok(())
    }

    fn serialize_amount(
        &mut self,
        field_id: FieldId<{ type_code::AMOUNT_6 }>,
        amount: Amount,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_amount(amount)?;
        Ok(())
    }

    fn serialize_blob(
        &mut self,
        field_id: FieldId<{ type_code::BLOB_7 }>,
        blob: &Blob,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_blob(blob)?;
        Ok(())
    }

    fn serialize_hash128(
        &mut self,
        field_id: FieldId<{ type_code::HASH128_4 }>,
        hash128: Hash128,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_hash128(hash128)?;
        Ok(())
    }

    fn serialize_hash160(
        &mut self,
        field_id: FieldId<{ type_code::HASH160_17 }>,
        hash160: Hash160,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_hash160(hash160)?;
        Ok(())
    }

    fn serialize_hash256(
        &mut self,
        field_id: FieldId<{ type_code::HASH256_5 }>,
        hash256: Hash256,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_hash256(hash256)?;
        Ok(())
    }

    fn serialize_uint8(
        &mut self,
        field_id: FieldId<{ type_code::UINT8_16 }>,
        uint8: UInt8,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_uint8(uint8)?;
        Ok(())
    }

    fn serialize_uint16(
        &mut self,
        field_id: FieldId<{ type_code::UINT16_1 }>,
        uint16: UInt16,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_uint16(uint16)?;
        Ok(())
    }

    fn serialize_uint32(
        &mut self,
        field_id: FieldId<{ type_code::UINT32_2 }>,
        uint32: UInt32,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_uint32(uint32)?;
        Ok(())
    }

    fn serialize_uint64(
        &mut self,
        field_id: FieldId<{ type_code::UINT64_3 }>,
        uint64: Uint64,
    ) -> Result<(), BinaryCodecError> {
        self.push_field_id(field_id)?;
        self.push_uint64(uint64)?;
        Ok(())
    }
}

impl<W: Write> Serializer<W> {
    fn push(&mut self, value: u8) -> Result<(), BinaryCodecError> {
        self.push_slice(&[value])
    }

    fn push_slice(&mut self, bytes: &[u8]) -> Result<(), BinaryCodecError> {
        self.writer.write_all(bytes)?;
        Ok(())
    }

    fn push_uint8(&mut self, value: UInt8) -> Result<(), BinaryCodecError> {
        self.push(value)
    }

    fn push_uint16(&mut self, value: UInt16) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.to_be_bytes())
    }

    fn push_uint32(&mut self, value: UInt32) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.to_be_bytes())
    }

    fn push_uint64(&mut self, value: Uint64) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.to_be_bytes())
    }

    fn push_hash128(&mut self, value: Hash128) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.0)
    }

    fn push_hash160(&mut self, value: Hash160) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.0)
    }

    fn push_hash256(&mut self, value: Hash256) -> Result<(), BinaryCodecError> {
        self.push_slice(&value.0)
    }

    fn push_blob(&mut self, blob: &Blob) -> Result<(), BinaryCodecError> {
        self.push_vl_prefix(blob.0.len())?;
        self.push_slice(&blob.0)?;
        Ok(())
    }

    /// Push field id <https://xrpl.org/serialization.html#field-ids>
    fn push_field_id<const TC: u8>(
        &mut self,
        field_id: FieldId<TC>,
    ) -> Result<(), BinaryCodecError> {
        // rippled implementation: https://github.com/seelabs/rippled/blob/cecc0ad75849a1d50cc573188ad301ca65519a5b/src/ripple/protocol/impl/Serializer.cpp#L117-L148

        let type_code = TC;
        let field_code = field_id.0 .0;
        if type_code < 16 && field_code < 16 {
            self.push(type_code << 4 | field_code)?;
        } else if type_code < 16 {
            self.push(type_code << 4)?;
            self.push(field_code)?;
        } else if field_code < 16 {
            self.push(field_code)?;
            self.push(type_code)?;
        } else {
            self.push(0)?;
            self.push(type_code)?;
            self.push(field_code)?;
        }
        Ok(())
    }

    /// Push length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    fn push_vl_prefix(&mut self, length: usize) -> Result<(), BinaryCodecError> {
        if length <= 192 {
            self.push(length as u8)?;
            Ok(())
        } else if length <= 12480 {
            let length = length - 193;
            self.push(193 + (length >> 8) as u8)?;
            self.push((length & 0xff) as u8)?;
            Ok(())
        } else if length <= 918744 {
            let length = length - 12481;
            self.push(241 + (length >> 16) as u8)?;
            self.push(((length >> 8) & 0xff) as u8)?;
            self.push((length & 0xff) as u8)?;
            Ok(())
        } else {
            Err(BinaryCodecError::OutOfRange(format!(
                "Variable length out of range: {}",
                length
            )))
        }
    }

    /// <https://xrpl.org/serialization.html#amount-fields>
    fn push_drops_amount(&mut self, drops: DropsAmount) -> Result<(), BinaryCodecError> {
        const POSITIVE_MASK: u64 = 0x4000000000000000;
        self.push_uint64(POSITIVE_MASK | drops.drops())
    }

    /// <https://xrpl.org/serialization.html#issued-currency-amount-format>
    fn push_issued_value(&mut self, value: IssuedValue) -> Result<(), BinaryCodecError> {
        const ISSUED_MASK: u64 = 0x8000000000000000;
        const POSITIVE_MASK: u64 = 0x4000000000000000;

        let (mantissa, positive) = match value.mantissa() {
            0 => {
                self.push_uint64(ISSUED_MASK)?;
                return Ok(());
            }
            1.. => (value.mantissa() as u64, true),
            ..=-1 => (-value.mantissa() as u64, false),
        };
        let exponent = (value.exponent() + 97) as u64;
        self.push_uint64(
            ISSUED_MASK | (if positive { POSITIVE_MASK } else { 0 }) | mantissa | (exponent << 54),
        )?;
        Ok(())
    }

    fn push_amount(&mut self, amount: Amount) -> Result<(), BinaryCodecError> {
        match amount {
            Amount::Drops(drops) => self.push_drops_amount(drops),
            Amount::Issued(issued) => {
                self.push_issued_value(issued.value())?;
                self.push_currency_code(issued.currency())?;
                self.push_account_id_no_length_prefix(issued.issuer())?;
                Ok(())
            }
        }
    }

    /// <https://xrpl.org/serialization.html#currency-codes>
    fn push_currency_code(&mut self, currency_code: CurrencyCode) -> Result<(), BinaryCodecError> {
        match currency_code {
            CurrencyCode::Xrp => self.push_slice(&[0u8; 20]),
            CurrencyCode::Standard(code) => {
                self.push_slice(&[0u8; 12])?;
                self.push_slice(&code.as_bytes())?;
                self.push_slice(&[0u8; 5])?;
                Ok(())
            }
            CurrencyCode::NonStandard(code) => self.push_slice(code.as_bytes()),
        }
    }

    fn push_account_id(&mut self, id: AccountId) -> Result<(), BinaryCodecError> {
        self.push_vl_prefix(20).expect("20 is within valid range");
        self.push_slice(&id.0)
    }

    fn push_account_id_no_length_prefix(&mut self, id: AccountId) -> Result<(), BinaryCodecError> {
        self.push_slice(&id.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_id::FieldCode;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;

    fn serializer() -> Serializer<Vec<u8>> {
        Serializer::new(Vec::new())
    }

    #[test]
    fn test_push_uint8() {
        let mut s = serializer();
        let value = 0x12;
        s.push_uint8(value).unwrap();
        assert_eq!(s.into_inner(), [0x12u8]);
    }

    #[test]
    fn test_push_uint16() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 8);
        s.push_uint16(value).unwrap();
        assert_eq!(s.into_inner(), [0x34, 0x12]);
    }

    #[test]
    fn test_push_uint32() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 24);
        s.push_uint32(value).unwrap();
        assert_eq!(s.into_inner(), [0x34, 0x00, 0x00, 0x12]);
    }

    #[test]
    fn test_push_uint64() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 56);
        s.push_uint64(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12]
        );
    }

    #[test]
    fn test_push_h128() {
        let mut s = serializer();
        let value = Hash128([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x12,
        ]);
        s.push_hash128(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_h160() {
        let mut s = serializer();
        let value = Hash160([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_hash160(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_h256() {
        let mut s = serializer();
        let value = Hash256([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_hash256(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_blob() {
        let mut s = serializer();
        let value = Blob(vec![0x34, 0x00, 0x12]);
        s.push_blob(&value).unwrap();
        assert_eq!(s.into_inner(), [3, 0x34, 0x00, 0x12]);
    }

    #[test]
    fn test_push_account_id() {
        let mut s = serializer();
        let value = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_account_id(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                20, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_account_id_no_length_prefix() {
        let mut s = serializer();
        let value = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_account_id_no_length_prefix(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    /// Tests length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    #[test]
    fn test_push_vl_prefix() {
        // test range 0 to 192
        let mut s = serializer();
        s.push_vl_prefix(0).unwrap();
        s.push_vl_prefix(1).unwrap();
        s.push_vl_prefix(192).unwrap();
        assert_eq!(s.into_inner(), [0, 1, 192]);

        // test range 193 to 12480
        let mut s = serializer();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 0).unwrap();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 1).unwrap();
        assert_eq!(193 + ((240 - 193) * 256) + 255, 12480);
        s.push_vl_prefix(193 + ((240 - 193) * 256) + 255).unwrap();
        assert_eq!(s.into_inner(), [193, 0, 193, 1, 240, 255]);

        // test range 12481 to 918744
        let mut s = serializer();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (0 * 256) + 0)
            .unwrap();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (0 * 256) + 1)
            .unwrap();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (1 * 256) + 0)
            .unwrap();
        s.push_vl_prefix(12481 + ((241 - 241) * 65536) + (255 * 256) + 255)
            .unwrap();
        assert_eq!(12481 + ((254 - 241) * 65536) + (212 * 256) + 23, 918744);
        s.push_vl_prefix(12481 + ((254 - 241) * 65536) + (212 * 256) + 23)
            .unwrap();
        assert_eq!(
            s.into_inner(),
            [241, 0, 0, 241, 0, 1, 241, 1, 0, 241, 255, 255, 254, 212, 23]
        );

        // test out of range
        let mut s = serializer();
        let result = s.push_vl_prefix(918745);
        assert_matches!(result, Err(BinaryCodecError::OutOfRange(message)) => {
            assert!(message.contains("Variable length out of range"), "message: {}", message);
        });
    }

    #[test]
    fn test_push_currency_code_xrp() {
        let mut s = serializer();
        let code = CurrencyCode::xrp();
        s.push_currency_code(code).unwrap();
        assert_eq!(s.into_inner(), [0u8; 20]);
    }

    #[test]
    fn test_push_currency_code_standard() {
        let mut s = serializer();
        let code = CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap();
        s.push_currency_code(code).unwrap();
        let bytes = s.into_inner();
        assert_eq!(bytes[0..12], [0u8; 12]);
        assert_eq!(
            bytes[12..15],
            [
                AsciiChar::U.as_byte(),
                AsciiChar::S.as_byte(),
                AsciiChar::D.as_byte()
            ]
        );
        assert_eq!(bytes[15..20], [0u8; 5]);
    }

    #[test]
    fn test_push_currency_code_non_standard() {
        let mut s = serializer();
        let code = CurrencyCode::non_standard([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ])
        .unwrap();
        s.push_currency_code(code).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            ]
        );
    }

    #[test]
    fn test_push_drops_amount() {
        let mut s = serializer();
        let value = DropsAmount::from_drops(10_000).unwrap();
        s.push_drops_amount(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]
        );
    }

    /// Test serializing zero issued value
    #[test]
    fn test_push_issued_value_zero() {
        let mut s = serializer();
        let value = IssuedValue::zero();
        s.push_issued_value(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]
        );
    }

    /// Test serializing positive issued value
    #[test]
    fn test_push_issued_value_positive() {
        let mut s = serializer();
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value).unwrap();
        let bytes = s.into_inner();
        assert_eq!(
            bytes,
            [0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00,],
            "actual: {}",
            hex::encode(&bytes),
        );
    }

    /// Test serializing negative issued value
    #[test]
    fn test_push_issued_value_negative() {
        let mut s = serializer();
        let value = IssuedValue::from_mantissa_exponent(-1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value).unwrap();
        let bytes = s.into_inner();
        assert_eq!(
            bytes,
            [0x95, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00,],
            "actual: {}",
            hex::encode(&bytes),
        );
    }

    #[test]
    fn test_push_amount_drops() {
        let mut s = serializer();
        let value = Amount::drops(10_000).unwrap();
        s.push_amount(value).unwrap();
        assert_eq!(
            s.into_inner(),
            [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]
        );
    }

    #[test]
    fn test_push_amount_issued() {
        let mut s = serializer();
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap();
        let currency = CurrencyCode::non_standard([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ])
        .unwrap();
        let issuer = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        let amount = Amount::issued(value, currency, issuer).unwrap();
        s.push_amount(amount).unwrap();
        let bytes = s.into_inner();
        assert_eq!(
            bytes,
            [
                0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ],
            "actual: {}",
            hex::encode(&bytes),
        );
    }

    #[test]
    fn test_push_field_id_4bit_type_4bit_field() {
        let mut s = serializer();
        let field_id = FieldId::uint32_2(FieldCode(0b0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0b0010_0100]);
    }

    #[test]
    fn test_push_field_id_4bit_type_8bit_field() {
        let mut s = serializer();
        let field_id = FieldId::uint32_2(FieldCode(0b0001_0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0b0010_0000, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_8bit_field() {
        let mut s = serializer();
        let field_id = FieldId::hash160_17(FieldCode(0b0001_0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0, 0b0001_0001, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_4bit_field() {
        let mut s = serializer();
        let field_id = FieldId::hash160_17(FieldCode(0b0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(s.into_inner(), [0b0000_0100, 0b0001_0001]);
    }

    /// Test serialize fields (in correct order)
    #[test]
    fn test_serialize_fields() {
        let mut s = serializer();
        let field_id = FieldId::uint32_2(FieldCode(1));
        s.serialize_uint32(field_id, 12).unwrap();
        let field_id = FieldId::uint32_2(FieldCode(2));
        s.serialize_uint32(field_id, 23).unwrap();
        let field_id = FieldId::uint64_3(FieldCode(1));
        s.serialize_uint64(field_id, 34).unwrap();
        assert_eq!(
            s.into_inner(),
            [
                0b0010_0001,
                0,
                0,
                0,
                12,
                0b0010_0010,
                0,
                0,
                0,
                23,
                0b0011_0001,
                0,
                0,
                0,
                0,
                0,
                0,
                0,
                34
            ]
        );
    }

    /// Test serialize fields where field ordering is wrong
    #[test]
    fn test_serialize_fields_wrong_type_code_order() {
        let mut s = serializer();
        let field_id = FieldId::uint64_3(FieldCode(1));
        s.serialize_uint64(field_id, 34).unwrap();
        let field_id = FieldId::uint32_2(FieldCode(2));
        let result = s.serialize_uint32(field_id, 12);
        assert_matches!(result, Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Variable length out of range"), "message: {}", message);
        });
    }

    /// Test serialize fields where field ordering is wrong
    #[test]
    fn test_serialize_fields_wrong_field_code_order() {
        let mut s = serializer();
        let field_id = FieldId::uint32_2(FieldCode(2));
        let result = s.serialize_uint32(field_id, 12);
        let field_id = FieldId::uint32_2(FieldCode(1));
        s.serialize_uint32(field_id, 34).unwrap();
        assert_matches!(result, Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Variable length out of range"), "message: {}", message);
        });
    }
}
