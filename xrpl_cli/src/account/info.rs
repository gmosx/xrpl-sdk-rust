use clap::ArgMatches;
use xrpl_sdk_jsonrpc::{AccountInfoRequest, Client};

pub fn account_info(account_matches: &ArgMatches, info_matches: &ArgMatches) {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();
        // TODO: also use account from environment.
        // TODO: render as text/md, html and json.
        // TODO: use handlebars for formatting?

        let resp = client.call(AccountInfoRequest::new(account)).await;

        if let Ok(resp) = resp {
            if info_matches.get_flag("json") {
                if info_matches.get_flag("pretty") {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(&resp.account_data).unwrap()
                    );
                } else {
                    println!("{}", serde_json::to_string(&resp.account_data).unwrap());
                }
            } else {
                println!("{:?}", resp.account_data);
            }
        }
    });
}
