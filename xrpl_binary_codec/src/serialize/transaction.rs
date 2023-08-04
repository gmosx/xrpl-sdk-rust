use xrpl_types::Transaction;
use crate::BinaryCodecError;
use crate::serialize::Serialize;
use crate::serializer::Serializer;

impl Serialize for Transaction {
    fn serialize(&self, serializer: &mut Serializer) -> Result<(), BinaryCodecError> {
        todo!()
    }
}