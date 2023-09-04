use crate::{hash, serialize, BinaryCodecError};
use libsecp256k1::{Message, PublicKey, SecretKey};
use xrpl_types::serialize::Serialize;
use xrpl_types::{Blob, Transaction};

/// Sign given transaction with secp256k1 <https://xrpl.org/cryptographic-keys.html#signing-algorithms>
pub fn sign_transaction<T: Transaction + Serialize>(
    transaction: &mut T,
    public_key: &PublicKey,
    secret_key: &SecretKey,
) -> Result<(), BinaryCodecError> {
    transaction.common_mut().signing_pub_key =
        Some(Blob(public_key.serialize_compressed().to_vec()));
    let serialized = serialize::serialize(transaction)?;
    let signature = signature(
        hash::HASH_PREFIX_UNSIGNED_TRANSACTION_SINGLE,
        &serialized,
        secret_key,
    );
    transaction.common_mut().txn_signature = Some(signature);
    Ok(())
}

/// Calculate secp256k1 signature <https://xrpl.org/cryptographic-keys.html#signing-algorithms>
fn signature(prefix: [u8; 4], data: &[u8], secret_key: &SecretKey) -> Blob {
    let hash = hash::hash(prefix, data);
    let message = Message::parse(&hash.0);
    let (signature, _) = libsecp256k1::sign(&message, secret_key);
    Blob(signature.serialize_der().as_ref().to_vec())
}

// todo allan
//
// #[cfg(test)]
// mod tests {
//     use xrpl_types::{AccountId, Amount, DropsAmount, Transaction, TransactionType};
//
//     use crate::{
//         sign::{sign, sign_transaction},
//         util::serialize_transaction_to_hex,
//     };
//
//     #[test]
//     fn test_serialize() {
//         let public_key = "037D37332B158AC75D7BA8E7EF1F3F4C7C0FA7B4BD8818B9C03545D3AED40BB3A9";
//
//         let public_key = hex::decode(public_key).unwrap();
//
//         let tx = Transaction {
//             transaction_type: TransactionType::Payment,
//             account: AccountId::from_address("rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe").unwrap(),
//             amount: Some(Amount::drops(22_000_000).unwrap()),
//             destination: Some(
//                 AccountId::from_address("rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe").unwrap(),
//             ),
//             flags: Some(2147483648),
//             last_ledger_sequence: Some(18311743),
//             fee: Some(DropsAmount::from_drops(12).unwrap()),
//             sequence: Some(18311659),
//             signing_public_key: Some(public_key),
//             signature: None,
//             memos: None,
//             offer_sequence: None,
//             taker_gets: None,
//             taker_pays: None,
//             expiration: None,
//             limit_amount: None,
//             quality_in: None,
//             quality_out: None,
//         };
//
//         let tx_hex = serialize_transaction_to_hex(&tx);
//
//         assert_eq!(tx_hex, "120000228000000024011769EB201B01176A3F6140000000014FB18068400000000000000C7321037D37332B158AC75D7BA8E7EF1F3F4C7C0FA7B4BD8818B9C03545D3AED40BB3A9811471CFCE39CE9B97E7E519AF8B282DDBE140A278748314F667B0CA50CC7709A220B0561B85E53A48461FA8");
//     }
//
//     #[test]
//     fn test_sign_transaction() {
//         let public_key = "037D37332B158AC75D7BA8E7EF1F3F4C7C0FA7B4BD8818B9C03545D3AED40BB3A9";
//         let secret_key = "165F2F406B5DCC37E666B7A0C9686CD4C92B67D5D362C618A96627E394F2FF45";
//
//         let public_key = hex::decode(public_key).unwrap();
//         let secret_key = hex::decode(secret_key).unwrap();
//
//         let tx = Transaction {
//             transaction_type: TransactionType::Payment,
//             account: AccountId::from_address("rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe").unwrap(),
//             amount: Some(Amount::drops(22_000_000).unwrap()),
//             destination: Some(
//                 AccountId::from_address("rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe").unwrap(),
//             ),
//             flags: Some(2147483648),
//             last_ledger_sequence: Some(18311743),
//             fee: Some(DropsAmount::from_drops(12).unwrap()),
//             sequence: Some(18311659),
//             signing_public_key: None,
//             signature: None,
//             memos: None,
//             offer_sequence: None,
//             taker_gets: None,
//             taker_pays: None,
//             expiration: None,
//             limit_amount: None,
//             quality_in: None,
//             quality_out: None,
//         };
//
//         let tx = sign_transaction(tx, &public_key, &secret_key);
//
//         let tx_hex = serialize_transaction_to_hex(&tx);
//
//         assert_eq!(tx_hex, "120000228000000024011769EB201B01176A3F6140000000014FB18068400000000000000C7321037D37332B158AC75D7BA8E7EF1F3F4C7C0FA7B4BD8818B9C03545D3AED40BB3A974463044022059E8475EF21F380A0A8FF70FF976F53DFB2EEAADD98860F642BF4004A008BEF7022014279499218DD1460B753135AEAED5A63935ACE5975869C3204886B1F346569E811471CFCE39CE9B97E7E519AF8B282DDBE140A278748314F667B0CA50CC7709A220B0561B85E53A48461FA8");
//     }
//
//     #[test]
//     fn test_sign() {
//         let tx = hex::decode("deadbeaf").unwrap();
//
//         // INSIGHT: The secret key is 32 bytes long. Remove the first byte (2 hex chars)
//         // if the key is padded to 33 bytes.
//         let key = hex::decode("915EDE054B37DF14BA612E7528A95B0D73013DC0ADED094B10957AD9BAD25455")
//             .unwrap();
//
//         let signature = sign(&tx, &key);
//
//         let signature_hex = hex::encode(signature).to_uppercase();
//
//         assert_eq!(signature_hex, "304402204228E8035C09363EA32C8A8E2BBBBFA4FA095ACB6415AF2D1E43E7237315F8220220233205F4D8F310EA172782057030CF65966864859CC1AD364B5BE8FD16C243BC");
//
//         println!("{}", signature_hex);
//     }
// }
// todo allan
