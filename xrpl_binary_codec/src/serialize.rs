use crate::error::BinaryCodecError;
use crate::serializer::Serializer;

pub trait Serialize {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), BinaryCodecError>;
}
