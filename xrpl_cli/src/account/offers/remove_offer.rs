use clap::ArgMatches;

use xrpl_binary_codec::{sign::sign_transaction, util::serialize_transaction_to_hex};
use xrpl_sdk_jsonrpc::{Client, SubmitRequest};
use xrpl_types::Transaction;

pub async fn remove_offer(
    account: impl AsRef<str>,
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

    let public_key = std::env::var("XRPL_ACC_PK").unwrap();
    let secret_key = std::env::var("XRPL_ACC_SK").unwrap();

    let public_key = hex::decode(public_key)?;
    let secret_key = hex::decode(secret_key)?;

    let tx = sign_transaction(tx, &public_key, &secret_key);

    let tx_blob = serialize_transaction_to_hex(&tx);

    let req = SubmitRequest::new(&tx_blob);
    let _resp = client.call(req).await?;

    Ok(())
}
