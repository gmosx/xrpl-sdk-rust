use clap::ArgMatches;
use libsecp256k1::{PublicKey, SecretKey};

use xrpl_binary_codec::{serialize, sign};
use xrpl_http_client::{Client, SubmitRequest};
use xrpl_types::{AccountId, OfferCreateTransaction, Transaction};

use crate::fmt::amount_from_str;

// xrpl account <ADDRESS> --public-key="..." --secret-key="..." offers create --taker-pays="5.0 USD:rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq" --taker-gets="1.0 XRP"

pub async fn create_offer(
    account: impl AsRef<str>,
    public_key: impl AsRef<str>,
    secret_key: impl AsRef<str>,
    matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account = account.as_ref();

    let taker_pays_spec: &String = matches.get_one("TAKER_PAYS").expect("taker pays missing");
    let taker_gets_spec: &String = matches.get_one("TAKER_GETS").expect("taker gets missing");

    let client = Client::new();

    let taker_pays = amount_from_str(taker_pays_spec).expect("invalid taker pays amount");
    let taker_gets = amount_from_str(taker_gets_spec).expect("invalid taker gets amount");

    // #warning this is an order from the TAKER side!
    let mut tx =
        OfferCreateTransaction::new(AccountId::from_address(account)?, taker_pays, taker_gets);

    client.prepare_transaction(tx.common_mut()).await?;

    // #insight
    // The secret/private key is 32 bytes, the public key is 33 bytes.

    let secret_key = SecretKey::parse_slice(&hex::decode(secret_key.as_ref())?)?;
    let public_key =
        PublicKey::parse_compressed(&hex::decode(public_key.as_ref())?.as_slice().try_into()?)?;

    sign::sign_transaction(&mut tx, &public_key, &secret_key)?;

    let tx_blob = serialize::serialize(&tx)?;

    let req = SubmitRequest::new(hex::encode(&tx_blob));
    let resp = client.call(req).await?;

    println!("{resp}");

    Ok(())
}
