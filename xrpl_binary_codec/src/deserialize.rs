use crate::deserializer::XrplDeserializer;
use crate::BinaryCodecError;
use crate::serializer::field_info::field_info_lookup;
use xrpl_types::deserialize::Deserialize;

/// Deserializes the given bytes in the canonical binary format into an object
/// Reference: <https://xrpl.org/serialization.html>
pub fn deserialize<T: Deserialize>(bytes: &[u8]) -> Result<T, BinaryCodecError> {
    let mut d = XrplDeserializer::new(bytes.to_vec(), field_info_lookup());
    T::deserialize(&mut d)
}
