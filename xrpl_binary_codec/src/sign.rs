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

#[cfg(test)]
mod tests {
    use super::*;
    use enumflags2::BitFlags;
    use xrpl_types::{AccountId, Amount, DropsAmount, PaymentTransaction};

    #[test]
    fn test_sign_transaction() {
        let public_key = "037D37332B158AC75D7BA8E7EF1F3F4C7C0FA7B4BD8818B9C03545D3AED40BB3A9";
        let secret_key = "165F2F406B5DCC37E666B7A0C9686CD4C92B67D5D362C618A96627E394F2FF45";

        let secret_key = SecretKey::parse_slice(&hex::decode(secret_key).unwrap()).unwrap();
        let public_key = PublicKey::parse_compressed(
            &hex::decode(public_key)
                .unwrap()
                .as_slice()
                .try_into()
                .unwrap(),
        )
        .unwrap();

        let mut tx = PaymentTransaction::new(
            AccountId::from_address("rB48JG388ovDA9fmPJbqgnSK3tnndSxgAe").unwrap(),
            Amount::drops(22_000_000).unwrap(),
            AccountId::from_address("rPT1Sjq2YGrBMTttX4GZHjKu9dyfzbpAYe").unwrap(),
        );
        tx.flags = BitFlags::from_bits(2147483648).unwrap();
        tx.common.last_ledger_sequence = Some(18311743);
        tx.common.fee = Some(DropsAmount::from_drops(12).unwrap());
        tx.common.sequence = Some(18311659);

        sign_transaction(&mut tx, &public_key, &secret_key).unwrap();

        let tx_hex = hex::encode_upper(serialize::serialize(&tx).unwrap());

        assert_eq!(tx_hex, "120000228000000024011769EB201B01176A3F6140000000014FB18068400000000000000C7321037D37332B158AC75D7BA8E7EF1F3F4C7C0FA7B4BD8818B9C03545D3AED40BB3A974463044022059E8475EF21F380A0A8FF70FF976F53DFB2EEAADD98860F642BF4004A008BEF7022014279499218DD1460B753135AEAED5A63935ACE5975869C3204886B1F346569E811471CFCE39CE9B97E7E519AF8B282DDBE140A278748314F667B0CA50CC7709A220B0561B85E53A48461FA8");
    }
}
