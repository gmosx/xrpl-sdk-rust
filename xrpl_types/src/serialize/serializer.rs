use crate::serialize::Serialize;
use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt8, Uint64};
use std::error::Error;

pub trait Serializer {
    type Error: Error;
    type SerializeArray<'a>: SerializeArray<Error = Self::Error>
    where
        Self: 'a;

    fn serialize_account_id(
        &mut self,
        field_name: &str,
        account_id: AccountId,
    ) -> Result<(), Self::Error>;

    fn serialize_amount(&mut self, field_name: &str, amount: Amount) -> Result<(), Self::Error>;

    fn serialize_blob(&mut self, field_name: &str, blob: &Blob) -> Result<(), Self::Error>;

    fn serialize_hash128(&mut self, field_name: &str, hash128: Hash128) -> Result<(), Self::Error>;

    fn serialize_hash160(&mut self, field_name: &str, hash160: Hash160) -> Result<(), Self::Error>;

    fn serialize_hash256(&mut self, field_name: &str, hash256: Hash256) -> Result<(), Self::Error>;

    fn serialize_uint8(&mut self, field_name: &str, uint8: UInt8) -> Result<(), Self::Error>;

    fn serialize_uint16(&mut self, field_name: &str, uint16: UInt16) -> Result<(), Self::Error>;

    fn serialize_uint32(&mut self, field_name: &str, uint32: UInt32) -> Result<(), Self::Error>;

    fn serialize_uint64(&mut self, field_name: &str, uint64: Uint64) -> Result<(), Self::Error>;

    fn serialize_array(
        &mut self,
        field_name: &str,
    ) -> Result<Self::SerializeArray<'_>, Self::Error>;
}

pub trait SerializeArray {
    type Error: Error;

    fn serialize_object<T: Serialize>(
        &mut self,
        field_name: &str,
        object: &T,
    ) -> Result<(), Self::Error>;

    fn end(self) -> Result<(), Self::Error>;
}
