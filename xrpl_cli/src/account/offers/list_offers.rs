use clap::ArgMatches;
use prettytable::row;
use prettytable::Table;
use xrpl_sdk_jsonrpc::{AccountOffersRequest, Client};

use crate::fmt::format_amount;

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
        let mut table = Table::new();

        table.add_row(row!["Sequence", "Taker Pays", "Taker Gets"]);

        let mut offers = resp.offers;
        offers.sort();

        for offer in offers {
            table.add_row(row![
                offer.seq,
                format_amount(&offer.taker_pays),
                format_amount(&offer.taker_gets)
            ]);
        }

        println!("{table}");
    } else {
        println!("{:?}", resp.offers);
    }

    Ok(())
}
