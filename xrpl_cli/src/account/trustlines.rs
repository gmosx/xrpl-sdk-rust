use clap::ArgMatches;
use xrpl_sdk_jsonrpc::Client;

pub fn account_trustlines(account_matches: &ArgMatches, lines_matches: &ArgMatches) {
    let account = account_matches.value_of("ACCOUNT").unwrap();

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();
        // TODO: add limit option
        // TODO: also use account from environment.
        // TODO: render as text/md, html and json.
        // TODO: use handlebars for formatting?

        let resp = client.account_lines(account).send().await;

        if let Ok(resp) = resp {
            if lines_matches.is_present("json") {
                if lines_matches.is_present("pretty") {
                    println!("{}", serde_json::to_string_pretty(&resp.lines).unwrap());
                } else {
                    println!("{}", serde_json::to_string(&resp.lines).unwrap());
                }
            } else if lines_matches.is_present("pretty") {
                for offer in resp.lines {
                    println!("{offer:?}");
                }
            } else {
                println!("{:?}", resp.lines);
            }
        }
    });
}
