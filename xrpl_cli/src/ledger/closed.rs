use clap::ArgMatches;
use xrpl_sdk_jsonrpc::Client;

pub fn ledger_closed(_ledger_matches: &ArgMatches) {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let client = Client::new();
        // TODO: render as text/md, html and json.
        // TODO: use handlebars for formatting?

        let resp = client.ledger_closed().send().await;

        if let Ok(resp) = resp {
            println!("{}", serde_json::to_string_pretty(&resp).unwrap());
        }
    });
}
