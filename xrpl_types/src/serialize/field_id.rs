pub type TypeCode = u8;

/// Field data type codes <https://xrpl.org/serialization.html#type-list>
pub mod type_code {
    use super::TypeCode;

    pub const ACCOUNT_ID_8: TypeCode = 8;
    pub const AMOUNT_6: TypeCode = 6;
    pub const BLOB_7: TypeCode = 7;
    pub const HASH128_4: TypeCode = 4;
    pub const HASH160_17: TypeCode = 17;
    pub const HASH256_5: TypeCode = 5;
    pub const UINT8_16: TypeCode = 16;
    pub const UINT16_1: TypeCode = 1;
    pub const UINT32_2: TypeCode = 2;
    pub const UINT64_3: TypeCode = 3;
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
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldCode(pub u8);

/// Type safe representation of field id <https://xrpl.org/serialization.html#canonical-field-order>
/// where different field type codes results in different field id Rust types.
#[derive(Debug, Clone, Copy)]
pub struct FieldId<const TC: TypeCode>(
    /// Field code <https://xrpl.org/serialization.html#field-codes>
    pub FieldCode,
);

impl<const TC: TypeCode> FieldId<TC> {
    pub fn field_code(self) -> FieldCode {
        self.0
    }

    pub fn type_code(self) -> TypeCode {
        TC
    }

    pub fn ord(self) -> FieldIdOrd {
        FieldIdOrd {
            type_code: TC,
            field_code: self.0,
        }
    }
}

/// Ordered field id <https://xrpl.org/serialization.html#canonical-field-order>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldIdOrd {
    /// Type code <https://xrpl.org/serialization.html#type-codes>
    pub type_code: TypeCode,
    /// Field code <https://xrpl.org/serialization.html#field-codes>
    pub field_code: FieldCode,
}

use type_code::*;

impl FieldId<ACCOUNT_ID_8> {
    pub fn account_id_8(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<AMOUNT_6> {
    pub fn amount_6(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<BLOB_7> {
    pub fn blob_7(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<HASH128_4> {
    pub fn hash128_4(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<HASH160_17> {
    pub fn hash160_17(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<HASH256_5> {
    pub fn hash256_5(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT8_16> {
    pub fn uint8_16(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT16_1> {
    pub fn uint16_1(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT32_2> {
    pub fn uint32_2(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}

impl FieldId<UINT64_3> {
    pub fn uint64_3(field_code: FieldCode) -> Self {
        Self(field_code)
    }
}
