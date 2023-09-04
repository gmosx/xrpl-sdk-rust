use clap::ArgMatches;
use libsecp256k1::{PublicKey, SecretKey};

use xrpl_binary_codec::{serialize, sign};
use xrpl_http_client::{Client, SubmitRequest};
use xrpl_types::{AccountId, OfferCancelTransaction, Transaction};

// xrpl account <ADDRESS> --public-key="..." --secret-key="..." offers remove <OFFER_SEQUENCE>

pub async fn remove_offer(
    account: impl AsRef<str>,
    public_key: impl AsRef<str>,
    secret_key: impl AsRef<str>,
    remove_offer_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account = account.as_ref();

    let offer_sequence: &String = remove_offer_matches
        .get_one("OFFER_SEQ")
        .expect("offer sequence missing");
    let offer_sequence: u32 = offer_sequence.parse().expect("offer sequence invalid");

    let client = Client::new();

    let mut tx =
        OfferCancelTransaction::new(AccountId::from_address(account.as_ref())?, offer_sequence);

    client.prepare_transaction(tx.common_mut()).await?;

    // #insight
    // The secret/private key is 32 bytes, the public key is 33 bytes.

    let secret_key = SecretKey::parse_slice(&hex::decode(secret_key.as_ref())?)?;
    let public_key = PublicKey::parse_compressed(&hex::decode(public_key.as_ref())?.as_slice().try_into()?)?;

    sign::sign_transaction(&mut tx, &public_key, &secret_key)?;

    let tx_blob = serialize::serialize(&tx)?;

    let req = SubmitRequest::new(hex::encode(&tx_blob));
    let resp = client.call(req).await?;

    println!("{resp}");

    Ok(())
}
