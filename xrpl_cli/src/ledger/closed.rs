use clap::ArgMatches;
use xrpl_sdk_jsonrpc::{Client, LedgerClosedRequest};

pub async fn ledger_closed(_ledger_matches: &ArgMatches) -> anyhow::Result<()> {
    let client = Client::new();

    // #todo render as text/md, html and json.
    // #todo use handlebars for formatting?

    let resp = client.call(LedgerClosedRequest::new()).await?;

    println!("{}", serde_json::to_string_pretty(&resp).unwrap());

    Ok(())
}
