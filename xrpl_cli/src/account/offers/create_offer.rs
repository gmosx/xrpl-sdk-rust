use clap::ArgMatches;

use xrpl_binary_codec::{sign::sign_transaction, util::serialize_transaction_to_hex};
use xrpl_sdk_jsonrpc::{Client, SubmitRequest};
use xrpl_types::Transaction;

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
    let tx = Transaction::offer_create(&account, taker_pays, taker_gets);

    let tx = client.prepare_transaction(tx).await?;

    // #insight
    // The secret/private key is 32 bytes, the public key is 33 bytes.

    let public_key = public_key.as_ref();
    let secret_key = secret_key.as_ref();

    let public_key = hex::decode(public_key)?;
    let secret_key = hex::decode(secret_key)?;

    let tx = sign_transaction(tx, &public_key, &secret_key);

    let tx_blob = serialize_transaction_to_hex(&tx);

    let req = SubmitRequest::new(&tx_blob);
    let resp = client.call(req).await?;

    println!("{resp}");

    Ok(())
}
