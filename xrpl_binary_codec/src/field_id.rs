/// Field data type codes <https://xrpl.org/serialization.html#type-list>
pub mod type_code {
    pub const ACCOUNT_ID: u8 = 8;
    pub const AMOUNT: u8 = 6;
    pub const BLOB: u8 = 7;
    pub const HASH128: u8 = 4;
    pub const HASH160: u8 = 17;
    pub const HASH256: u8 = 5;
    pub const UINT8: u8 = 16;
    pub const UINT16: u8 = 1;
    pub const UINT32: u8 = 2;
    pub const UINT64: u8 = 3;
}

// /// Field data type codes <https://xrpl.org/serialization.html#type-list>
// #[derive(Debug, Clone, Copy, Eq, PartialEq)]
// #[repr(u8)]
// pub enum TypeCode {
//     // Discriminant values can be found at https://xrpl.org/serialization.html#type-list and also at https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json
//     AccountId = 8,
//     Amount = 6,
//     Blob = 7,
//     Hash128 = 4,
//     Hash160 = 17,
//     Hash256 = 5,
//     UInt8 = 16,
//     UInt16 = 1,
//     UInt32 = 2,
//     UInt64 = 3,
// }
// todo allan

/// Field code <https://xrpl.org/serialization.html#field-codes>. The code for a given field can be found at
/// <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json> or
/// <https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L72-L266>
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FieldCode(pub u8);

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct FieldId<const TC: u8>(
    /// Field code <https://xrpl.org/serialization.html#field-codes>
    pub FieldCode,
);

use type_code::*;

impl FieldId<ACCOUNT_ID> {
    pub fn account_id(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<AMOUNT> {
    pub fn amount(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<BLOB> {
    pub fn blob(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<HASH128> {
    pub fn hash128(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<HASH160> {
    pub fn hash160(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<HASH256> {
    pub fn hash256(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT8> {
    pub fn uint8(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT16> {
    pub fn uint16(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT32> {
    pub fn uint32(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT64> {
    pub fn uint64(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

