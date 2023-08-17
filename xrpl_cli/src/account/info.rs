use clap::ArgMatches;
use xrpl_http_client::{AccountInfoRequest, Client};

pub async fn account_info(
    account_matches: &ArgMatches,
    info_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    let client = Client::new();
    // #todo also use account from environment.
    // #todo render as text/md, html and json.
    // #todo use handlebars for formatting?

    let resp = client.call(AccountInfoRequest::new(account)).await?;

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

    Ok(())
}
