use clap::ArgMatches;

use xrpl_binary_codec::{sign::sign_transaction, util::serialize_transaction_to_hex};
use xrpl_sdk_jsonrpc::{Client, SubmitRequest};
use xrpl_types::Transaction;

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

    let tx = Transaction::offer_cancel(account, offer_sequence);

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
