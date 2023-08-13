pub mod list_offers;
pub mod offer_info;
pub mod remove_offer;

use clap::ArgMatches;
use xrpl_sdk_jsonrpc::{AccountOffersRequest, Client};

use remove_offer::remove_offer;

pub fn account_offers(account_matches: &ArgMatches, offers_matches: &ArgMatches) {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    if let Some(remove_offer_matches) = offers_matches.subcommand_matches("remove") {
        remove_offer(account, remove_offer_matches);
    }

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();
        // TODO: add limit option
        // TODO: also use account from environment.
        // TODO: render as text/md, html and json.
        // TODO: use handlebars for formatting?

        let req = AccountOffersRequest::new(account);
        let resp = client.call(req).await;

        if let Ok(resp) = resp {
            if offers_matches.get_flag("json") {
                if offers_matches.get_flag("pretty") {
                    println!("{}", serde_json::to_string_pretty(&resp.offers).unwrap());
                } else {
                    println!("{}", serde_json::to_string(&resp.offers).unwrap());
                }
            } else if offers_matches.get_flag("pretty") {
                for offer in resp.offers {
                    println!("{offer:?}");
                }
            } else {
                println!("{:?}", resp.offers);
            }
        }
    });
}
