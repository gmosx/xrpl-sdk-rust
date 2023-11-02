use std::fmt;

use super::field_info::FieldInfo;

/// Field data type codes <https://xrpl.org/serialization.html#type-list>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
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
    Array = 15,
    Object = 14,
}

impl fmt::Display for TypeCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Field code <https://xrpl.org/serialization.html#field-codes>. The code for a given field can be found at
/// <https://github.com/XRPLF/xrpl.js/blob/main/packages/ripple-binary-codec/src/enums/definitions.json> or
/// <https://github.com/XRPLF/rippled/blob/72e6005f562a8f0818bc94803d222ac9345e1e40/src/ripple/protocol/impl/SField.cpp#L72-L266>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldCode(pub u8);

impl fmt::Display for FieldCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Ordered field id <https://xrpl.org/serialization.html#canonical-field-order>
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct FieldId {
    /// Type code <https://xrpl.org/serialization.html#type-codes>
    pub type_code: TypeCode,
    /// Field code <https://xrpl.org/serialization.html#field-codes>
    pub field_code: FieldCode,
}

impl FieldId {
    pub fn from_type_field(type_code: TypeCode, field_code: FieldCode) -> Self {
        Self {
            type_code,
            field_code,
        }
    }
}

impl From<FieldInfo> for FieldId {
    fn from(field_info: FieldInfo) -> Self {
        Self {
            type_code: field_info.field_type,
            field_code: field_info.field_code,
        }
    }
}

// rippled implementation: https://github.com/seelabs/rippled/blob/cecc0ad75849a1d50cc573188ad301ca65519a5b/src/ripple/protocol/impl/Serializer.cpp#L117-L148
impl Into<Vec<u8>> for FieldId {
    fn into(self) -> Vec<u8> {
        let mut header = Vec::new();

        let type_code = self.type_code as u8;
        let field_code = self.field_code.0;

        if type_code < 16 && field_code < 16 {
            header.push(type_code << 4 | field_code);
        } else if type_code < 16 {
            header.push(type_code << 4);
            header.push(field_code);
        } else if field_code < 16 {
            header.push(field_code);
            header.push(type_code);
        } else {
            header.push(0);
            header.push(type_code);
            header.push(field_code);
        }
        header
    }
}
