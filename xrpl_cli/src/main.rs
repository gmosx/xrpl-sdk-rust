mod account;
mod ledger;

use crate::account::offers::account_offers;
use account::{balances::account_balances, info::account_info};
use clap::{Arg, Command};
use ledger::closed::ledger_closed;

// TODO: introduce `xrpl_util` or `xrpl_fmt` crate.

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    tracing_subscriber::fmt::init();

    // `account` subcommand

    let account_cmd = Command::new("account")
        .about("Account")
        .arg(
            // Positional argument.
            Arg::new("ACCOUNT")
                .help("The address of the account")
                .required(true)
                .index(1),
        )
        .subcommand(
            Command::new("info")
                .about("info")
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Format response as JSON")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty-print the response")
                        .takes_value(false),
                ),
        )
        .subcommand(
            Command::new("balances")
                .about("balances")
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Format response as JSON")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty-print the response")
                        .takes_value(false),
                ),
        )
        .subcommand(
            Command::new("offers")
                .about("offers")
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("LIMIT")
                        .help("The maximum count of offers returned")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Format response as JSON")
                        .takes_value(false),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty-print the response")
                        .takes_value(false),
                ),
        );

    // `ledger` subcommand.

    let ledger_cmd = Command::new("ledger")
        .about("Ledger")
        .arg(
            Arg::new("index")
                .short('i')
                .long("index")
                .value_name("LEDGER_INDEX")
                .help("Selects the ledger by index")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new("hash")
                .short('s') // #Note `-h` conflict with `--help`.
                .long("hash")
                .value_name("LEDGER_HASH")
                .help("Selects the ledger by hash")
                .required(false)
                .takes_value(true),
        )
        .arg(
            Arg::new("closed")
                .short('c')
                .long("closed")
                .help("Selects the latest closed ledger")
                .required(false)
                .takes_value(false),
        );

    let mut xrpl_cmd = Command::new("xrpl")
        .author("George Moschovitis, gmosx@reizu.org")
        .version(VERSION)
        .about("A CLI for the XRP Ledger")
        .after_help(
            "The xrpl CLI provides access to ledger and account data on the XRP Ledger and allows for signing transactions.",
        )
        .subcommand(account_cmd)
        .subcommand(ledger_cmd);

    let matches = xrpl_cmd.clone().get_matches();

    if let Some(account_matches) = matches.subcommand_matches("account") {
        if let Some(info_matches) = account_matches.subcommand_matches("info") {
            account_info(account_matches, info_matches);
        } else if let Some(balance_matches) = account_matches.subcommand_matches("balances") {
            account_balances(account_matches, balance_matches);
        } else if let Some(offers_matches) = account_matches.subcommand_matches("offers") {
            account_offers(account_matches, offers_matches);
        }
    } else if let Some(ledger_matches) = matches.subcommand_matches("ledger") {
        // #TODO properly handle this
        ledger_closed(ledger_matches);
    } else {
        xrpl_cmd.print_long_help().unwrap();
    }
}
