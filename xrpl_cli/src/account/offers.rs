use clap::ArgMatches;
use xrpl_sdk_jsonrpc::Client;

pub fn account_offers(account_matches: &ArgMatches, offers_matches: &ArgMatches) {
    let account = account_matches.value_of("ACCOUNT").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();
        // TODO: add limit option
        // TODO: also use account from environment.
        // TODO: render as text/md, html and json.
        // TODO: use handlebars for formatting?

        let resp = client.account_offers(account).send().await;

        if let Ok(resp) = resp {
            if offers_matches.is_present("json") {
                if offers_matches.is_present("pretty") {
                    println!("{}", serde_json::to_string_pretty(&resp.offers).unwrap());
                } else {
                    println!("{}", serde_json::to_string(&resp.offers).unwrap());
                }
            } else if offers_matches.is_present("pretty") {
                for offer in resp.offers {
                    println!("{offer:?}");
                }
            } else {
                println!("{:?}", resp.offers);
            }
        }
    });
}
