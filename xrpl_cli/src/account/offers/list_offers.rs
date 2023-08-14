use clap::ArgMatches;
use xrpl_sdk_jsonrpc::{AccountOffersRequest, Client};

pub async fn list_offers(
    account: impl AsRef<str>,
    list_offers_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account = account.as_ref();

    let client = Client::new();
    // TODO: add limit option
    // TODO: also use account from environment.
    // TODO: render as text/md, html and json.
    // TODO: use handlebars for formatting?

    let req = AccountOffersRequest::new(account);
    let resp = client.call(req).await?;

    if list_offers_matches.get_flag("json") {
        if list_offers_matches.get_flag("pretty") {
            println!("{}", serde_json::to_string_pretty(&resp.offers).unwrap());
        } else {
            println!("{}", serde_json::to_string(&resp.offers).unwrap());
        }
    } else if list_offers_matches.get_flag("pretty") {
        for offer in resp.offers {
            println!("{offer:?}");
        }
    } else {
        println!("{:?}", resp.offers);
    }

    Ok(())
}
