// use super::serializer::Serializer;
// use xrpl_types::Transaction;
//
// pub fn serialize_transaction(tx: &Transaction) -> Vec<u8> {
//     let mut s = Serializer::new();
//     s.push_transaction(tx, None);
//     s.to_vec()
// }
//
// pub fn serialize_transaction_to_hex(tx: &Transaction) -> String {
//     let mut s = Serializer::new();
//     s.push_transaction(tx, None);
//     hex::encode(s.to_vec()).to_uppercase()
// }
//
// todo allan
