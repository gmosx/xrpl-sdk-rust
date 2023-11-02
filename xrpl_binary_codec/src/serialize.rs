use crate::alloc::vec::Vec;
use crate::serializer::Serializer;
use crate::BinaryCodecError;
use xrpl_types::serialize::Serialize;

/// Serializes the given object in the canonical binary format <https://xrpl.org/serialization.html>
pub fn serialize(object: &impl Serialize) -> Result<Vec<u8>, BinaryCodecError> {
    let mut s = Serializer::new();
    object.serialize(&mut s)?;
    s.into_bytes()
}
