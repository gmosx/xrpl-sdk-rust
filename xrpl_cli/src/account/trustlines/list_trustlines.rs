use clap::ArgMatches;
use prettytable::row;
use prettytable::Table;
use xrpl_http_client::{AccountLinesRequest, Client};

pub async fn list_trustlines(
    account: impl AsRef<str>,
    list_trustlines_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account = account.as_ref();

    let client = Client::new();
    // #todo add limit option
    // #todo also use account from environment.
    // #todo render as text/md, html and json.
    // #todo use handlebars for formatting?

    let req = AccountLinesRequest::new(account);
    let resp = client.call(req).await?;

    if list_trustlines_matches.get_flag("json") {
        if list_trustlines_matches.get_flag("pretty") {
            println!("{}", serde_json::to_string_pretty(&resp.lines).unwrap());
        } else {
            println!("{}", serde_json::to_string(&resp.lines).unwrap());
        }
    } else if list_trustlines_matches.get_flag("pretty") {
        let mut table = Table::new();

        table.add_row(row!["Currency", "Account", "Balance", "Limit", "No Ripple"]);

        let lines = resp.lines;

        let lines_count = lines.len();

        for line in lines {
            table.add_row(row![
                line.currency,
                line.account,
                line.balance,
                line.limit,
                line.no_ripple.unwrap_or_default(),
            ]);
        }

        println!("{table}{} lines.", lines_count);
    } else {
        println!("{:?}", resp.lines);
    }

    Ok(())
}
