use crate::{AccountId, Amount, Blob, Hash128, Hash160, Hash256, UInt16, UInt32, UInt8, Uint64};
use std::error::Error;

pub trait Deserializer {
    type Error: Error;
    type DeserializeArray: Sized;
    type DeserializeObject: Sized;

    fn deserialize_account_id(&mut self) -> Result<AccountId, Self::Error>;

    fn deserialize_amount(&mut self) -> Result<Amount, Self::Error>;

    fn deserialize_blob(&mut self, len: usize) -> Result<Blob, Self::Error>;

    fn deserialize_hash128(&mut self) -> Result<Hash128, Self::Error>;

    fn deserialize_hash160(&mut self) -> Result<Hash160, Self::Error>;

    fn deserialize_hash256(&mut self) -> Result<Hash256, Self::Error>;

    fn deserialize_uint8(&mut self) -> Result<UInt8, Self::Error>;

    fn deserialize_uint16(&mut self) -> Result<UInt16, Self::Error>;

    fn deserialize_uint32(&mut self) -> Result<UInt32, Self::Error>;

    fn deserialize_uint64(&mut self) -> Result<Uint64, Self::Error>;

    fn deserialize_array(&mut self) -> Result<Self::DeserializeArray, Self::Error>;

    fn deserialize_object(&mut self) -> Result<Self::DeserializeObject, Self::Error>;
}
