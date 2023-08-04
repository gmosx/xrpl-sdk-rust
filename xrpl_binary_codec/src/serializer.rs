use crate::error::BinaryCodecError;
use crate::field_id::FieldId;
use xrpl_types::Uint64;
use xrpl_types::{
    AccountId, Amount, Blob, CurrencyCode, DropsAmount, Hash128, Hash160, Hash256, IssuedAmount,
    IssuedValue, UInt16, UInt32, UInt8,
};

pub const HASH_PREFIX_TRANSACTION: [u8; 4] = [0x53, 0x4E, 0x44, 0x00];
pub const HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE: [u8; 4] = [0x53, 0x54, 0x58, 0x00];

#[derive(Default)]
pub struct Serializer {
    pub buf: Vec<u8>,
}

impl Serializer {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn serialize_account_id(
        &mut self,
        field_id: FieldId,
        account_id: &AccountId,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_blob(
        &mut self,
        field_id: FieldId,
        blob: &Blob,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_hash128(
        &mut self,
        field_id: FieldId,
        hash128: Hash128,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_hash160(
        &mut self,
        field_id: FieldId,
        hash160: Hash160,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_hash256(
        &mut self,
        field_id: FieldId,
        hash256: Hash256,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_uint8(
        &mut self,
        field_id: FieldId,
        uint8: UInt8,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_uint16(
        &mut self,
        field_id: FieldId,
        uint16: UInt16,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }
    pub fn serialize_uint32(
        &mut self,
        field_id: FieldId,
        uint32: UInt32,
    ) -> Result<(), BinaryCodecError> {
        todo!()
    }

    fn push(&mut self, value: u8) {
        self.buf.push(value);
    }

    fn push_slice(&mut self, bytes: &[u8]) {
        self.buf.extend(bytes);
    }

    fn push_uint8(&mut self, value: UInt8) {
        self.push(value);
    }

    fn push_uint16(&mut self, value: UInt16) {
        self.push_slice(&value.to_be_bytes());
    }

    fn push_uint32(&mut self, value: UInt32) {
        self.push_slice(&value.to_be_bytes());
    }

    fn push_uint64(&mut self, value: Uint64) {
        self.push_slice(&value.to_be_bytes());
    }

    fn push_hash128(&mut self, value: Hash128) {
        self.push_slice(&value.0);
    }

    fn push_hash160(&mut self, value: Hash160) {
        self.push_slice(&value.0);
    }

    fn push_hash256(&mut self, value: Hash256) {
        self.push_slice(&value.0);
    }

    fn push_blob(&mut self, blob: Blob) -> Result<(), BinaryCodecError> {
        self.push_vl_prefix(blob.0.len())?;
        self.push_slice(&blob.0);
        Ok(())
    }

    /// Push field id <https://xrpl.org/serialization.html#field-ids>
    fn push_field_id(&mut self, field_id: FieldId) {
        // rippled implementation: https://github.com/seelabs/rippled/blob/cecc0ad75849a1d50cc573188ad301ca65519a5b/src/ripple/protocol/impl/Serializer.cpp#L117-L148

        let type_code = field_id.type_code as u8;
        let field_code = field_id.field_code.0;
        if type_code < 16 && field_code < 16 {
            self.push(type_code << 4 | field_code);
        } else if type_code < 16 {
            self.push(type_code << 4);
            self.push(field_code);
        } else if field_code < 16 {
            self.push(field_code);
            self.push(type_code);
        } else {
            self.push(0);
            self.push(field_id.type_code as u8);
            self.push(field_id.field_code.0);
        }
    }

    /// Push length prefix according to <https://xrpl.org/serialization.html#length-prefixing>
    fn push_vl_prefix(&mut self, length: usize) -> Result<(), BinaryCodecError> {
        if length <= 192 {
            self.push(length as u8);
            Ok(())
        } else if length <= 12480 {
            let length = length - 193;
            self.push(193 + (length >> 8) as u8);
            self.push((length & 0xff) as u8);
            Ok(())
        } else if length <= 918744 {
            let length = length - 12481;
            self.push(241 + (length >> 16) as u8);
            self.push(((length >> 8) & 0xff) as u8);
            self.push((length & 0xff) as u8);
            Ok(())
        } else {
            Err(BinaryCodecError::OutOfRange(format!(
                "Variable length out of range: {}",
                length
            )))
        }
    }

    /// <https://xrpl.org/serialization.html#amount-fields>
    fn push_drops_amount(&mut self, drops: DropsAmount) {
        const POSITIVE_MASK: u64 = 0x4000000000000000;
        self.push_uint64(POSITIVE_MASK | drops.drops());
    }

    /// <https://xrpl.org/serialization.html#issued-currency-amount-format>
    fn push_issued_value(&mut self, value: IssuedValue) {
        const ISSUED_MASK: u64 = 0x8000000000000000;
        const POSITIVE_MASK: u64 = 0x4000000000000000;

        let (mantissa, positive) = match value.mantissa() {
            0 => {
                self.push_uint64(ISSUED_MASK);
                return;
            }
            1.. => (value.mantissa() as u64, true),
            ..=-1 => (-value.mantissa() as u64, false),
        };
        let exponent = (value.exponent() + 97) as u64;
        self.push_uint64(
            ISSUED_MASK | (if positive { POSITIVE_MASK } else { 0 }) | mantissa | (exponent << 54),
        );
    }

    fn push_amount(&mut self, amount: Amount) {
        match amount {
            Amount::Drops(drops) => self.push_drops_amount(drops),
            Amount::Issued(issued) => {
                self.push_issued_value(issued.value());
                self.push_currency_code(issued.currency());
                self.push_account_id_no_length_prefix(issued.issuer());
            }
        }
    }

    /// <https://xrpl.org/serialization.html#currency-codes>
    fn push_currency_code(&mut self, currency_code: CurrencyCode) {
        match currency_code {
            CurrencyCode::Xrp => self.push_slice(&[0u8; 20]),
            CurrencyCode::Standard(code) => {
                self.push_slice(&[0u8; 12]);
                self.push_slice(&code.as_bytes());
                self.push_slice(&[0u8; 5]);
            }
            CurrencyCode::NonStandard(code) => self.push_slice(code.as_bytes()),
        }
    }

    fn push_account_id(&mut self, id: AccountId) {
        self.push_vl_prefix(20).expect("20 is within valid range");
        self.push_slice(&id.0);
    }

    fn push_account_id_no_length_prefix(&mut self, id: AccountId) {
        self.push_slice(&id.0);
    }

    pub fn to_vec(&self) -> Vec<u8> {
        self.buf.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::AsciiChar;
    use ascii::AsciiChar::i;
    use assert_matches::assert_matches;
    use crate::field_id::{FieldCode, TypeCode};

    #[test]
    fn test_push_uint8() {
        let mut s = Serializer::new();
        let value = 0x12;
        s.push_uint8(value);
        assert_eq!(s.buf, [0x12u8]);
    }

    #[test]
    fn test_push_uint16() {
        let mut s = Serializer::new();
        let value = 0x12 + (0x34 << 8);
        s.push_uint16(value);
        assert_eq!(s.buf, [0x34, 0x12]);
    }

    #[test]
    fn test_push_uint32() {
        let mut s = Serializer::new();
        let value = 0x12 + (0x34 << 24);
        s.push_uint32(value);
        assert_eq!(s.buf, [0x34, 0x00, 0x00, 0x12]);
    }

    #[test]
    fn test_push_uint64() {
        let mut s = Serializer::new();
        let value = 0x12 + (0x34 << 56);
        s.push_uint64(value);
        assert_eq!(s.buf, [0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12]);
    }

    #[test]
    fn test_push_h128() {
        let mut s = Serializer::new();
        let value = Hash128([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x12,
        ]);
        s.push_hash128(value);
        assert_eq!(
            s.buf,
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_h160() {
        let mut s = Serializer::new();
        let value = Hash160([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_hash160(value);
        assert_eq!(
            s.buf,
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_h256() {
        let mut s = Serializer::new();
        let value = Hash256([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_hash256(value);
        assert_eq!(
            s.buf,
            [
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_blob() {
        let mut s = Serializer::new();
        let value = Blob(vec![0x34, 0x00, 0x12]);
        s.push_blob(value).unwrap();
        assert_eq!(s.buf, [3, 0x34, 0x00, 0x12]);
    }

    #[test]
    fn test_push_account_id() {
        let mut s = Serializer::new();
        let value = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_account_id(value);
        assert_eq!(
            s.buf,
            [
                20, 0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ]
        );
    }

    #[test]
    fn test_push_account_id_no_length_prefix() {
        let mut s = Serializer::new();
        let value = AccountId([
            0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x12,
        ]);
        s.push_account_id_no_length_prefix(value);
        assert_eq!(
            s.buf,
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
        let mut s = Serializer::new();
        s.push_vl_prefix(0).unwrap();
        s.push_vl_prefix(1).unwrap();
        s.push_vl_prefix(192).unwrap();
        assert_eq!(s.buf, [0, 1, 192]);

        // test range 193 to 12480
        let mut s = Serializer::new();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 0).unwrap();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 1).unwrap();
        assert_eq!(193 + ((240 - 193) * 256) + 255, 12480);
        s.push_vl_prefix(193 + ((240 - 193) * 256) + 255).unwrap();
        assert_eq!(s.buf, [193, 0, 193, 1, 240, 255]);

        // test range 12481 to 918744
        let mut s = Serializer::new();
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
            s.buf,
            [241, 0, 0, 241, 0, 1, 241, 1, 0, 241, 255, 255, 254, 212, 23]
        );

        // test out of range
        let mut s = Serializer::new();
        let result = s.push_vl_prefix(918745);
        assert_matches!(result, Err(BinaryCodecError::OutOfRange(message)) => {
            assert!(message.contains("Variable length out of range"), "message: {}", message);
        });
    }

    #[test]
    fn test_push_currency_code_xrp() {
        let mut s = Serializer::new();
        let code = CurrencyCode::xrp();
        s.push_currency_code(code);
        assert_eq!(s.buf, [0u8; 20]);
    }

    #[test]
    fn test_push_currency_code_standard() {
        let mut s = Serializer::new();
        let code = CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap();
        s.push_currency_code(code);
        assert_eq!(s.buf[0..12], [0u8; 12]);
        assert_eq!(
            s.buf[12..15],
            [
                AsciiChar::U.as_byte(),
                AsciiChar::S.as_byte(),
                AsciiChar::D.as_byte()
            ]
        );
        assert_eq!(s.buf[15..20], [0u8; 5]);
    }

    #[test]
    fn test_push_currency_code_non_standard() {
        let mut s = Serializer::new();
        let code = CurrencyCode::non_standard([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ])
        .unwrap();
        s.push_currency_code(code);
        assert_eq!(
            s.buf,
            [
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            ]
        );
    }

    #[test]
    fn test_push_drops_amount() {
        let mut s = Serializer::new();
        let value = DropsAmount::from_drops(10_000).unwrap();
        s.push_drops_amount(value);
        assert_eq!(s.buf, [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]);
    }

    /// Test serializing zero issued value
    #[test]
    fn test_push_issued_value_zero() {
        let mut s = Serializer::new();
        let value = IssuedValue::zero();
        s.push_issued_value(value);
        assert_eq!(s.buf, [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    /// Test serializing positive issued value
    #[test]
    fn test_push_issued_value_positive() {
        let mut s = Serializer::new();
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value);
        assert_eq!(
            s.buf,
            [0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00,],
            "actual: {}",
            hex::encode(&s.buf),
        );
    }

    /// Test serializing negative issued value
    #[test]
    fn test_push_issued_value_negative() {
        let mut s = Serializer::new();
        let value = IssuedValue::from_mantissa_exponent(-1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value);
        assert_eq!(
            s.buf,
            [0x95, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00,],
            "actual: {}",
            hex::encode(&s.buf),
        );
    }

    #[test]
    fn test_push_amount_drops() {
        let mut s = Serializer::new();
        let value = Amount::drops(10_000).unwrap();
        s.push_amount(value);
        assert_eq!(s.buf, [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]);
    }

    #[test]
    fn test_push_amount_issued() {
        let mut s = Serializer::new();
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
        s.push_amount(amount);
        assert_eq!(
            s.buf,
            [
                0xD5, 0xC3, 0x8D, 0x7E, 0xA4, 0xC6, 0x80, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
                0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x12
            ],
            "actual: {}",
            hex::encode(&s.buf),
        );
    }


    #[test]
    fn test_push_field_id_4bit_type_4bit_field() {
        let mut s = Serializer::new();
        let field_id = FieldId::new(TypeCode::UInt32, FieldCode(0b0100));
        s.push_field_id(field_id);
        assert_eq!(s.buf, [0b0010_0100]);
    }

    #[test]
    fn test_push_field_id_4bit_type_8bit_field() {
        let mut s = Serializer::new();
        let field_id = FieldId::new(TypeCode::UInt32, FieldCode(0b0001_0100));
        s.push_field_id(field_id);
        assert_eq!(s.buf, [0b0010_0000, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_8bit_field() {
        let mut s = Serializer::new();
        let field_id = FieldId::new(TypeCode::Hash160, FieldCode(0b0001_0100));
        s.push_field_id(field_id);
        assert_eq!(s.buf, [0, 0b0001_0001, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_4bit_field() {
        let mut s = Serializer::new();
        let field_id = FieldId::new(TypeCode::Hash160, FieldCode(0b0100));
        s.push_field_id(field_id);
        assert_eq!(s.buf, [0b0000_0100, 0b0001_0001]);
    }
}
