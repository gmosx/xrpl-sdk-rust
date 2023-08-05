use crate::serialize::{type_code, FieldId};
use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt8, Uint64};
use std::error::Error;

pub trait Serializer {
    type Error: Error;

    fn serialize_account_id(
        &mut self,
        field_id: FieldId<{ type_code::ACCOUNT_ID_8 }>,
        account_id: AccountId,
    ) -> Result<(), Self::Error>;

    fn serialize_amount(
        &mut self,
        field_id: FieldId<{ type_code::AMOUNT_6 }>,
        amount: Amount,
    ) -> Result<(), Self::Error>;

    fn serialize_blob(
        &mut self,
        field_id: FieldId<{ type_code::BLOB_7 }>,
        blob: &Blob,
    ) -> Result<(), Self::Error>;

    fn serialize_hash128(
        &mut self,
        field_id: FieldId<{ type_code::HASH128_4 }>,
        hash128: Hash128,
    ) -> Result<(), Self::Error>;

    fn serialize_hash160(
        &mut self,
        field_id: FieldId<{ type_code::HASH160_17 }>,
        hash160: Hash160,
    ) -> Result<(), Self::Error>;

    fn serialize_hash256(
        &mut self,
        field_id: FieldId<{ type_code::HASH256_5 }>,
        hash256: Hash256,
    ) -> Result<(), Self::Error>;

    fn serialize_uint8(
        &mut self,
        field_id: FieldId<{ type_code::UINT8_16 }>,
        uint8: UInt8,
    ) -> Result<(), Self::Error>;

    fn serialize_uint16(
        &mut self,
        field_id: FieldId<{ type_code::UINT16_1 }>,
        uint16: UInt16,
    ) -> Result<(), Self::Error>;

    fn serialize_uint32(
        &mut self,
        field_id: FieldId<{ type_code::UINT32_2 }>,
        uint32: UInt32,
    ) -> Result<(), Self::Error>;

    fn serialize_uint64(
        &mut self,
        field_id: FieldId<{ type_code::UINT64_3 }>,
        uint64: Uint64,
    ) -> Result<(), Self::Error>;
}
