use clap::ArgMatches;
use xrpl_sdk_jsonrpc::Client;

pub fn account_info(account_matches: &ArgMatches, info_matches: &ArgMatches) {
    let account = account_matches.value_of("ACCOUNT").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();
        // TODO: also use account from environment.
        // TODO: render as text/md, html and json.
        // TODO: use handlebars for formatting?

        let resp = client.account_info(account).send().await;

        if let Ok(resp) = resp {
            if info_matches.is_present("json") {
                if info_matches.is_present("pretty") {
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
