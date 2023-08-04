use crate::error::BinaryCodecError;
use crate::serializer::SerializerT;

mod transaction;

pub trait Serialize {
    /// Serialize the object. Notice that fields must be serialized in the order given by
    /// <https://xrpl.org/serialization.html#canonical-field-order>
    fn serialize<S: SerializerT>(&self, serializer: &mut S) -> Result<(), BinaryCodecError>;
}
