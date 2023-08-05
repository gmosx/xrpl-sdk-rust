use crate::serialize::Serialize;
use crate::serializer::SerializerT;
use crate::BinaryCodecError;
use xrpl_types::TrustSetTransaction;

impl Serialize for TrustSetTransaction {
    fn serialize<S: SerializerT>(&self, serializer: &mut S) -> Result<(), BinaryCodecError> {
        // serializer.se
        todo!()
    }
}
