/// Field data type codes <https://xrpl.org/serialization.html#type-list>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u8)]
pub enum TypeCode {
    // Discriminant values can be found at https://xrpl.org/serialization.html#type-list and also at https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json
    AccountId = 8,
    Amount = 6,
    Blob = 7,
    Hash128 = 4,
    Hash160 = 17,
    Hash256 = 5,
    UInt8 = 16,
    UInt16 = 1,
    UInt32 = 2,
    UInt64 = 3,
}

/// Field code <https://xrpl.org/serialization.html#field-codes>. The code for a given field can be found at
/// <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json> or
/// <https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L72-L266>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FieldCode(pub u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FieldId {
    /// Type code <https://xrpl.org/serialization.html#type-list>
    pub type_code: TypeCode,
    /// Field code <https://xrpl.org/serialization.html#field-codes>
    pub field_code: FieldCode,
}

impl FieldId {

    pub fn new(type_code: TypeCode,
               field_code: FieldCode) -> Self {
        Self {
            type_code,
            field_code,
        }
    }

    pub fn account_id(field_code: FieldCode) -> Self {
        Self::new(TypeCode::AccountId, field_code)
    }

    pub fn amount(field_code: FieldCode) -> Self {
        Self::new(TypeCode::Amount, field_code)
    }

    pub fn blob(field_code: FieldCode) -> Self {
        Self::new(TypeCode::Blob, field_code)
    }

    pub fn hash128(field_code: FieldCode) -> Self {
        Self::new(TypeCode::Hash128, field_code)
    }

    pub fn hash160(field_code: FieldCode) -> Self {
        Self::new(TypeCode::Hash160, field_code)
    }

    pub fn hash256(field_code: FieldCode) -> Self {
        Self::new(TypeCode::Hash256, field_code)
    }

    pub fn uint8(field_code: FieldCode) -> Self {
        Self::new(TypeCode::UInt8, field_code)
    }

    pub fn uint16(field_code: FieldCode) -> Self {
        Self::new(TypeCode::UInt16, field_code)
    }

    pub fn uint32(field_code: FieldCode) -> Self {
        Self::new(TypeCode::UInt32, field_code)
    }

    pub fn uint64(field_code: FieldCode) -> Self {
        Self::new(TypeCode::UInt64, field_code)
    }


}
