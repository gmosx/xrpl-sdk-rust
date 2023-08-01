use crate::serializer::Serializer;
use crate::error::BinaryCodecError;

pub trait Serialize {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), BinaryCodecError>;
}