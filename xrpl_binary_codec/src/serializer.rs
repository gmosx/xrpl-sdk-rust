use crate::error::BinaryCodecError;
use xrpl_types::serialize::{FieldCode, FieldId, Serialize, SerializeArray, TypeCode};
use xrpl_types::Uint64;
use xrpl_types::{
    AccountId, Amount, Blob, CurrencyCode, DropsAmount, Hash128, Hash160, Hash256, IssuedValue,
    UInt16, UInt32, UInt8,
};

mod field_info;

// todo allan
pub const HASH_PREFIX_TRANSACTION: [u8; 4] = [0x53, 0x4E, 0x44, 0x00];
pub const HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE: [u8; 4] = [0x53, 0x54, 0x58, 0x00];

#[derive(Debug, Default)]
pub struct Serializer {
    /// Buffer in which fields are initially serialized. Fields are not sorted in this buffer
    buffer: Vec<u8>,
    /// Tracks which fields have been serialized to the buffer
    serialized_fields: Vec<SerializedFieldIndex>,
}

impl xrpl_types::serialize::Serializer for Serializer {
    type Error = BinaryCodecError;
    type SerializeArray<'a> = ArraySerializer<'a>;

    fn serialize_account_id(
        &mut self,
        field_name: &str,
        account_id: AccountId,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::AccountId, |ser| {
            ser.push_account_id(account_id)?;
            Ok(())
        })
    }

    fn serialize_amount(
        &mut self,
        field_name: &str,
        amount: Amount,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::Amount, |ser| {
            ser.push_amount(amount)?;
            Ok(())
        })
    }

    fn serialize_blob(&mut self, field_name: &str, blob: &Blob) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::Blob, |ser| {
            ser.push_blob(blob)?;
            Ok(())
        })
    }

    fn serialize_hash128(
        &mut self,
        field_name: &str,
        hash128: Hash128,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::Hash128, |ser| {
            ser.push_hash128(hash128)?;
            Ok(())
        })
    }

    fn serialize_hash160(
        &mut self,
        field_name: &str,
        hash160: Hash160,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::Hash160, |ser| {
            ser.push_hash160(hash160)?;
            Ok(())
        })
    }

    fn serialize_hash256(
        &mut self,
        field_name: &str,
        hash256: Hash256,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::Hash256, |ser| {
            ser.push_hash256(hash256)?;
            Ok(())
        })
    }

    fn serialize_uint8(&mut self, field_name: &str, uint8: UInt8) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::UInt8, |ser| {
            ser.push_uint8(uint8)?;
            Ok(())
        })
    }

    fn serialize_uint16(
        &mut self,
        field_name: &str,
        uint16: UInt16,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::UInt16, |ser| {
            ser.push_uint16(uint16)?;
            Ok(())
        })
    }

    fn serialize_uint32(
        &mut self,
        field_name: &str,
        uint32: UInt32,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::UInt32, |ser| {
            ser.push_uint32(uint32)?;
            Ok(())
        })
    }

    fn serialize_uint64(
        &mut self,
        field_name: &str,
        uint64: Uint64,
    ) -> Result<(), BinaryCodecError> {
        self.serialize_field(field_name, TypeCode::UInt64, |ser| {
            ser.push_uint64(uint64)?;
            Ok(())
        })
    }

    fn serialize_array(
        &mut self,
        field_name: &str,
    ) -> Result<Self::SerializeArray<'_>, Self::Error> {
        let start_index = self.start_field(field_name, TypeCode::Array)?;
        Ok(ArraySerializer {
            serializer: self,
            start_index,
        })
    }
}

#[derive(Debug)]
pub struct ArraySerializer<'a> {
    serializer: &'a mut Serializer,
    start_index: SerializeFieldStartIndex,
}

impl<'a> SerializeArray for ArraySerializer<'a> {
    type Error = BinaryCodecError;

    fn serialize_object<T: Serialize>(
        &mut self,
        field_name: &str,
        object: &T,
    ) -> Result<(), Self::Error> {
        let field_id = field_id(field_name, TypeCode::Object)?;
        self.serializer.push_field_id(field_id)?;
        let mut object_serializer = Serializer::new();
        object.serialize(&mut object_serializer)?;
        self.serializer
            .push_slice(&object_serializer.into_bytes()?)?;
        self.serializer
            .push_field_id(FieldId::from_type_field(TypeCode::Object, FieldCode(1)))?;
        Ok(())
    }

    fn end(self) -> Result<(), Self::Error> {
        self.serializer
            .push_field_id(FieldId::from_type_field(TypeCode::Array, FieldCode(1)))?;
        self.serializer.end_field(self.start_index);
        Ok(())
    }
}

/// Represents a field serialized to to the buffer
#[derive(Debug)]
struct SerializedFieldIndex {
    /// id of the serialized field
    field_id: FieldId,
    /// index of serialized bytes in buffer
    index: usize,
    /// length of serialized bytes
    length: usize,
}

/// Represents that serialization of a field to the buffer has been started
#[derive(Debug)]
struct SerializeFieldStartIndex {
    /// id of the serialized field
    field_id: FieldId,
    /// index of serialized bytes in buffer
    index: usize,
}

impl SerializeFieldStartIndex {
    fn new(field_id: FieldId, index: usize) -> Self {
        Self { field_id, index }
    }

    fn end(self, end_index: usize) -> SerializedFieldIndex {
        SerializedFieldIndex {
            field_id: self.field_id,
            index: self.index,
            length: end_index - self.index,
        }
    }
}

impl Serializer {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            serialized_fields: Vec::new(),
        }
    }

    pub fn into_bytes(self) -> Result<Vec<u8>, BinaryCodecError> {
        let mut bytes = Vec::with_capacity(self.buffer.len());
        let mut serialized_fields = self.serialized_fields;
        serialized_fields.sort_by_key(|f| f.field_id);
        for field_pair in serialized_fields.windows(2) {
            if field_pair[0].field_id == field_pair[1].field_id {
                return Err(BinaryCodecError::FieldOrder(
                    "Two fields with same id".to_string(),
                ));
            }
        }
        for field in serialized_fields {
            bytes.extend_from_slice(&self.buffer[field.index..field.index + field.length]);
        }
        Ok(bytes)
    }

    fn start_field(
        &mut self,
        field_name: &str,
        field_type: TypeCode,
    ) -> Result<SerializeFieldStartIndex, BinaryCodecError> {
        let field_id = field_id(field_name, field_type)?;
        let start_index = SerializeFieldStartIndex::new(field_id, self.buffer.len());
        self.push_field_id(field_id)?;
        Ok(start_index)
    }

    fn end_field(&mut self, start_index: SerializeFieldStartIndex) {
        let index = start_index.end(self.buffer.len());
        self.serialized_fields.push(index);
    }

    fn serialize_field(
        &mut self,
        field_name: &str,
        field_type: TypeCode,
        serialize_closure: impl FnOnce(&mut Serializer) -> Result<(), BinaryCodecError>,
    ) -> Result<(), BinaryCodecError> {
        let start_index = self.start_field(field_name, field_type)?;
        serialize_closure(self)?;
        self.end_field(start_index);

        Ok(())
    }

    fn push(&mut self, value: u8) -> Result<(), BinaryCodecError> {
        self.push_slice(&[value])
    }

    fn push_slice(&mut self, bytes: &[u8]) -> Result<(), BinaryCodecError> {
        self.buffer.extend_from_slice(bytes);
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
    fn push_field_id(&mut self, field_id: FieldId) -> Result<(), BinaryCodecError> {
        // rippled implementation: https://github.com/seelabs/rippled/blob/cecc0ad75849a1d50cc573188ad301ca65519a5b/src/ripple/protocol/impl/Serializer.cpp#L117-L148

        let type_code = field_id.type_code as u8;
        let field_code = field_id.field_code.0;

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

pub fn field_id(field_name: &str, field_type: TypeCode) -> Result<FieldId, BinaryCodecError> {
    let field_info = field_info::field_info(field_name).ok_or_else(|| {
        BinaryCodecError::InvalidField(format!("Field with name {} is not known", field_name))
    })?;
    if field_type != field_info.field_type {
        return Err(BinaryCodecError::InvalidField(format!(
            "Field with name {} must have type {}",
            field_name, field_info.field_type
        )));
    }
    Ok(FieldId::from_type_field(
        field_info.field_type,
        field_info.field_code,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;
    use enumflags2::BitFlags;
    use xrpl_types::serialize::{FieldCode, Serialize, Serializer};
    use xrpl_types::OfferCreateTransaction;

    fn serializer() -> super::Serializer {
        super::Serializer::new()
    }

    fn buffer(serializer: &super::Serializer) -> &[u8] {
        &serializer.buffer
    }

    struct TestObject {
        field1: UInt32,
        field2: UInt32,
    }

    impl Serialize for TestObject {
        fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
            serializer.serialize_uint32("NetworkID", self.field1)?; // field code 1
            serializer.serialize_uint32("Flags", self.field2)?; // field code 2
            Ok(())
        }
    }

    struct TestObjectSerializeFieldsOutOfOrder {
        field1: UInt32,
        field2: UInt32,
    }

    impl Serialize for TestObjectSerializeFieldsOutOfOrder {
        fn serialize<S: Serializer>(&self, serializer: &mut S) -> Result<(), S::Error> {
            serializer.serialize_uint32("Flags", self.field2)?; // field code 2
            serializer.serialize_uint32("NetworkID", self.field1)?; // field code 1
            Ok(())
        }
    }

    #[test]
    fn test_push_uint8() {
        let mut s = serializer();
        let value = 0x12;
        s.push_uint8(value).unwrap();
        assert_eq!(buffer(&s), [0x12u8]);
    }

    #[test]
    fn test_push_uint16() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 8);
        s.push_uint16(value).unwrap();
        assert_eq!(buffer(&s), [0x34, 0x12]);
    }

    #[test]
    fn test_push_uint32() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 24);
        s.push_uint32(value).unwrap();
        assert_eq!(buffer(&s), [0x34, 0x00, 0x00, 0x12]);
    }

    #[test]
    fn test_push_uint64() {
        let mut s = serializer();
        let value = 0x12 + (0x34 << 56);
        s.push_uint64(value).unwrap();
        assert_eq!(buffer(&s), [0x34, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x12]);
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
            buffer(&s),
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
            buffer(&s),
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
            buffer(&s),
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
        assert_eq!(buffer(&s), [3, 0x34, 0x00, 0x12]);
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
            buffer(&s),
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
            buffer(&s),
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
        assert_eq!(buffer(&s), [0, 1, 192]);

        // test range 193 to 12480
        let mut s = serializer();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 0).unwrap();
        s.push_vl_prefix(193 + ((193 - 193) * 256) + 1).unwrap();
        assert_eq!(193 + ((240 - 193) * 256) + 255, 12480);
        s.push_vl_prefix(193 + ((240 - 193) * 256) + 255).unwrap();
        assert_eq!(buffer(&s), [193, 0, 193, 1, 240, 255]);

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
            buffer(&s),
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
        assert_eq!(buffer(&s), [0u8; 20]);
    }

    #[test]
    fn test_push_currency_code_standard() {
        let mut s = serializer();
        let code = CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap();
        s.push_currency_code(code).unwrap();
        let bytes = buffer(&s);
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
            buffer(&s),
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
        assert_eq!(buffer(&s), [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]);
    }

    /// Test serializing zero issued value
    #[test]
    fn test_push_issued_value_zero() {
        let mut s = serializer();
        let value = IssuedValue::zero();
        s.push_issued_value(value).unwrap();
        assert_eq!(buffer(&s), [0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    }

    /// Test serializing positive issued value
    #[test]
    fn test_push_issued_value_positive() {
        let mut s = serializer();
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, -10).unwrap();
        s.push_issued_value(value).unwrap();
        let bytes = buffer(&s);
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
        let bytes = buffer(&s);
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
        assert_eq!(buffer(&s), [0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x27, 0x10]);
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
        let bytes = buffer(&s);
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
        let field_id = FieldId::from_type_field(TypeCode::UInt32, FieldCode(0b0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(buffer(&s), [0b0010_0100]);
    }

    #[test]
    fn test_push_field_id_4bit_type_8bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::UInt32, FieldCode(0b0001_0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(buffer(&s), [0b0010_0000, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_8bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::Hash160, FieldCode(0b0001_0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(buffer(&s), [0, 0b0001_0001, 0b0001_0100]);
    }

    #[test]
    fn test_push_field_id_8bit_type_4bit_field() {
        let mut s = serializer();
        let field_id = FieldId::from_type_field(TypeCode::Hash160, FieldCode(0b0100));
        s.push_field_id(field_id).unwrap();
        assert_eq!(buffer(&s), [0b0000_0100, 0b0001_0001]);
    }

    /// Test pushing array of objects
    #[test]
    fn test_push_empty_array() {
        let mut s = serializer();
        let array = s.serialize_array("Memos").unwrap(); // field code 9
        array.end().unwrap();
        assert_eq!(buffer(&s), [0b1111_1001, 0b1111_0001]);
    }

    /// Test pushing array of objects
    #[test]
    fn test_push_array() {
        let mut s = serializer();
        let object1 = TestObject {
            field1: 12,
            field2: 23,
        };
        let object2 = TestObject {
            field1: 34,
            field2: 45,
        };
        let mut array = s.serialize_array("Memos").unwrap(); // field code 9
        array.serialize_object("Memo", &object1).unwrap(); // field code 10
        array.serialize_object("Memo", &object2).unwrap(); // field code 10
        array.end().unwrap();
        assert_eq!(
            buffer(&s),
            [
                0b1111_1001,
                0b1110_1010,
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
                0b1110_0001,
                0b1110_1010,
                0b0010_0001,
                0,
                0,
                0,
                34,
                0b0010_0010,
                0,
                0,
                0,
                45,
                0b1110_0001,
                0b1111_0001
            ]
        );
    }

    /// Test pushing array of objects with out of order fields
    #[test]
    fn test_push_array_out_of_order_fields() {
        let mut s = serializer();
        let object1 = TestObjectSerializeFieldsOutOfOrder {
            field1: 12,
            field2: 23,
        };
        let object2 = TestObjectSerializeFieldsOutOfOrder {
            field1: 34,
            field2: 45,
        };
        let mut array = s.serialize_array("Memos").unwrap(); // field code 9
        array.serialize_object("Memo", &object1).unwrap(); // field code 10
        array.serialize_object("Memo", &object2).unwrap(); // field code 10
        array.end().unwrap();
        assert_eq!(
            buffer(&s),
            [
                0b1111_1001,
                0b1110_1010,
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
                0b1110_0001,
                0b1110_1010,
                0b0010_0001,
                0,
                0,
                0,
                34,
                0b0010_0010,
                0,
                0,
                0,
                45,
                0b1110_0001,
                0b1111_0001
            ]
        );
    }

    /// Test serialize fields (in correct order)
    #[test]
    fn test_serialize_fields() {
        let mut s = serializer();
        s.serialize_uint32("NetworkID", 12).unwrap(); // field code 1
        s.serialize_uint32("Flags", 23).unwrap(); // field code 2
        s.serialize_uint64("IndexNext", 34).unwrap(); // field code 1
        assert_eq!(
            s.into_bytes().unwrap(),
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

    /// Test serialize fields out of order
    #[test]
    fn test_serialize_fields_type_code_order() {
        let mut s = serializer();
        s.serialize_uint64("IndexNext", 34).unwrap(); // field code 1
        s.serialize_uint32("NetworkID", 12).unwrap(); // field code 1
        assert_eq!(
            s.into_bytes().unwrap(),
            [
                0b0010_0001,
                0,
                0,
                0,
                12,
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

    /// Test serialize fields out of order
    #[test]
    fn test_serialize_fields_field_code_order() {
        let mut s = serializer();
        s.serialize_uint32("Flags", 23).unwrap(); // field code 2
        s.serialize_uint32("NetworkID", 12).unwrap(); // field code 1
        assert_eq!(
            s.into_bytes().unwrap(),
            [0b0010_0001, 0, 0, 0, 12, 0b0010_0010, 0, 0, 0, 23,]
        );
    }

    /// Test serialize fields where field ordering is wrong
    #[test]
    fn test_serialize_fields_same_field_id() {
        let mut s = serializer();
        s.serialize_uint32("Flags", 34).unwrap();
        let result = s.serialize_uint32("Flags", 12).and_then(|_| s.into_bytes());
        assert_matches!(result, Err(BinaryCodecError::FieldOrder(message)) => {
            assert!(message.contains("Two fields with same id"), "message: {}", message);
        });
    }

    /// Tests the example <https://xrpl.org/serialization.html#examples>
    #[test]
    fn test_serialize_offer_create() {
        let mut s = serializer();
        let mut tx = OfferCreateTransaction::new(
            AccountId::from_address("rMBzp8CgpE441cp5PVyA9rpVV7oT8hP3ys").unwrap(),
            Amount::drops(15000000000).unwrap(),
            Amount::issued(
                IssuedValue::from_mantissa_exponent(70728, -1).unwrap(),
                CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
                AccountId::from_address("rvYAfWj5gh67oV6fW32ZzP3Aw4Eubs59B").unwrap(),
            )
            .unwrap(),
        );
        tx.common.fee = Some(DropsAmount::from_drops(10).unwrap());
        tx.common.sequence = Some(1752792);
        tx.common.signing_pub_key = Some(Blob(
            hex::decode("03EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3")
                .unwrap(),
        ));
        tx.common.txn_signature = Some(Blob(hex::decode("30440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C").unwrap()));
        tx.expiration = Some(595640108);
        tx.flags = BitFlags::from_bits(524288).unwrap();
        tx.offer_sequence = Some(1752791);

        tx.serialize(&mut s).unwrap();
        assert_eq!(hex::encode_upper(s.into_bytes().unwrap()), "120007220008000024001ABED82A2380BF2C2019001ABED764D55920AC9391400000000000000000000000000055534400000000000A20B3C85F482532A9578DBB3950B85CA06594D165400000037E11D60068400000000000000A732103EE83BB432547885C219634A1BC407A9DB0474145D69737D09CCDC63E1DEE7FE3744630440220143759437C04F7B61F012563AFE90D8DAFC46E86035E1D965A9CED282C97D4CE02204CFD241E86F17E011298FC1A39B63386C74306A5DE047E213B0F29EFA4571C2C8114DD76483FACDEE26E60D8A586BB58D09F27045C46");
    }
}
