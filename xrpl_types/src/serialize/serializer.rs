use crate::serialize::{FieldCode, Serialize};
use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt8, Uint64};
use std::error::Error;

pub trait Serializer {
    type Error: Error;
    type SerializeArray<'a>: SerializeArray<Error = Self::Error>
    where
        Self: 'a;

    fn serialize_account_id(
        &mut self,
        field_code: FieldCode,
        account_id: AccountId,
    ) -> Result<(), Self::Error>;

    fn serialize_amount(
        &mut self,
        field_code: FieldCode,
        amount: Amount,
    ) -> Result<(), Self::Error>;

    fn serialize_blob(&mut self, field_code: FieldCode, blob: &Blob) -> Result<(), Self::Error>;

    fn serialize_hash128(
        &mut self,
        field_code: FieldCode,
        hash128: Hash128,
    ) -> Result<(), Self::Error>;

    fn serialize_hash160(
        &mut self,
        field_code: FieldCode,
        hash160: Hash160,
    ) -> Result<(), Self::Error>;

    fn serialize_hash256(
        &mut self,
        field_code: FieldCode,
        hash256: Hash256,
    ) -> Result<(), Self::Error>;

    fn serialize_uint8(&mut self, field_code: FieldCode, uint8: UInt8) -> Result<(), Self::Error>;

    fn serialize_uint16(
        &mut self,
        field_code: FieldCode,
        uint16: UInt16,
    ) -> Result<(), Self::Error>;

    fn serialize_uint32(
        &mut self,
        field_code: FieldCode,
        uint32: UInt32,
    ) -> Result<(), Self::Error>;

    fn serialize_uint64(
        &mut self,
        field_code: FieldCode,
        uint64: Uint64,
    ) -> Result<(), Self::Error>;

    fn serialize_array(
        &mut self,
        field_code: FieldCode,
    ) -> Result<Self::SerializeArray<'_>, Self::Error>;
}

pub trait SerializeArray {
    type Error: Error;

    fn serialize_object<T: Serialize>(
        &mut self,
        field_code: FieldCode,
        object: &T,
    ) -> Result<(), Self::Error>;

    fn end(self) -> Result<(), Self::Error>;
}
