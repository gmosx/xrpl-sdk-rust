use clap::ArgMatches;
use std::collections::HashMap;
use xrpl_sdk_jsonrpc::{AccountInfoRequest, AccountLinesRequest, Client};

// #TODO should be `balance` or `balances`?
// #TODO add error handling

pub async fn account_balances(
    account_matches: &ArgMatches,
    balances_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    let client = Client::new();
    // #todo also use account from environment.
    // #todo render as text/md, html and json.
    // #todo use handlebars for formatting?

    let mut balances: HashMap<String, f64> = HashMap::new();

    let account_info_resp = client.call(AccountInfoRequest::new(account)).await?;
    let account_lines_resp = client.call(AccountLinesRequest::new(account)).await?;

    let account_data = &account_info_resp.account_data;

    balances.insert(
        "XRP".to_owned(),
        account_data.balance.as_ref().unwrap().parse().unwrap(),
    );

    for line in account_lines_resp.lines {
        let iou = format!("{}.{}", line.currency, line.account);
        let iou_balance: f64 = line.balance.parse().unwrap();
        if iou_balance > 0.0 {
            balances.insert(iou, line.balance.parse().unwrap());
        }
    }

    if balances_matches.get_flag("json") {
        if balances_matches.get_flag("pretty") {
            println!("{}", serde_json::to_string_pretty(&balances).unwrap());
        } else {
            println!("{}", serde_json::to_string(&balances).unwrap());
        }
    } else {
        for (asset, balance) in balances {
            println!("{asset}: {balance}");
        }
    }

    Ok(())
}
