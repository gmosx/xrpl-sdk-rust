use xrpl_types::serialize::Serialize;
use crate::BinaryCodecError;
use crate::serializer::Serializer;

pub fn serialize(object: impl Serialize) -> Result<Vec<u8>, BinaryCodecError> {
    let mut s = Serializer::new();
    object.serialize(&mut s)?;
    s.into_bytes()
}