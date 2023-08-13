use clap::ArgMatches;
use xrpl_sdk_jsonrpc::{AccountLinesRequest, Client};

pub async fn account_trustlines(
    account_matches: &ArgMatches,
    lines_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    let client = Client::new();
    // TODO: add limit option
    // TODO: also use account from environment.
    // TODO: render as text/md, html and json.
    // TODO: use handlebars for formatting?

    let resp = client.call(AccountLinesRequest::new(account)).await?;

    if lines_matches.get_flag("json") {
        if lines_matches.get_flag("pretty") {
            println!("{}", serde_json::to_string_pretty(&resp.lines).unwrap());
        } else {
            println!("{}", serde_json::to_string(&resp.lines).unwrap());
        }
    } else if lines_matches.get_flag("pretty") {
        for offer in resp.lines {
            println!("{offer:?}");
        }
    } else {
        println!("{:?}", resp.lines);
    }

    Ok(())
}
