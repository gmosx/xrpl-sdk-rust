//! A CLI for the XRP Ledger.

mod account;
mod ledger;

use account::{
    balances::account_balances, info::account_info, offers::account_offers,
    trustlines::account_trustlines,
};
use clap::{Arg, ArgAction, Command};
use ledger::closed::ledger_closed;

// #TODO also used WebSocket
// #TODO introduce `xrpl_util` or `xrpl_fmt` crate.

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
        .arg(
            Arg::new("PUBLIC_KEY")
                .help("The public key of the account")
                .short('p')
                .long("public-key")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("SECRET_KEY")
                .help("The secret/private key of the account")
                .short('s')
                .long("secret-key")
                .required(false)
                .action(ArgAction::Set),
        )
        .subcommand(
            Command::new("info")
                .about("info")
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Format response as JSON")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty-print the response")
                        .action(ArgAction::SetTrue),
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
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty-print the response")
                        .action(ArgAction::SetTrue),
                ),
        )
        .subcommand(
            Command::new("offers")
                .about("Account offers")
                .subcommand(
                    Command::new("list")
                        .about("List account offers")
                        .arg(
                            Arg::new("limit")
                                .short('l')
                                .long("limit")
                                .value_name("LIMIT")
                                .help("The maximum count of offers returned")
                                .required(false)
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("json")
                                .short('j')
                                .long("json")
                                .help("Format response as JSON")
                                .action(ArgAction::SetTrue),
                        )
                        .arg(
                            Arg::new("pretty")
                                .short('p')
                                .long("pretty")
                                .help("Pretty-print the response")
                                .action(ArgAction::SetTrue),
                        ),
                )
                .subcommand(
                    Command::new("create")
                        .about("Create offer")
                        .arg(
                            Arg::new("TAKER_PAYS")
                                .short('p')
                                .long("taker-pays")
                                .help("The amount the taker pays")
                                .required(true)
                                .action(ArgAction::Set),
                        )
                        .arg(
                            Arg::new("TAKER_GETS")
                                .short('g')
                                .long("taker-gets")
                                .help("The amount the taker gets")
                                .required(true)
                                .action(ArgAction::Set),
                        ),
                )
                .subcommand(
                    Command::new("remove").about("Remove offer").arg(
                        // Positional argument.
                        Arg::new("OFFER_SEQ")
                            .help("The sequence of the offer")
                            .required(true)
                            .index(1),
                    ),
                ),
        )
        .subcommand(
            Command::new("trustlines")
                .about("lines")
                .arg(
                    Arg::new("limit")
                        .short('l')
                        .long("limit")
                        .value_name("LIMIT")
                        .help("The maximum count of trustlines returned")
                        .required(false)
                        .action(ArgAction::Set),
                )
                .arg(
                    Arg::new("json")
                        .short('j')
                        .long("json")
                        .help("Format response as JSON")
                        .action(ArgAction::SetTrue),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty-print the response")
                        .action(ArgAction::SetTrue),
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
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("hash")
                .short('s') // #Note `-h` conflict with `--help`.
                .long("hash")
                .value_name("LEDGER_HASH")
                .help("Selects the ledger by hash")
                .required(false)
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("closed")
                .short('c')
                .long("closed")
                .help("Selects the latest closed ledger")
                .required(false)
                .action(ArgAction::SetTrue),
        );

    let mut xrpl_cmd = Command::new("xrpl")
        .author("Georgios Moschovitis, george.moschovitis@gmail.com")
        .version(VERSION)
        .about("A CLI for the XRP Ledger")
        .after_help(
            "The xrpl CLI provides access to ledger and account data on the XRP Ledger and allows for signing transactions.",
        )
        .subcommand(account_cmd)
        .subcommand(ledger_cmd);

    let matches = xrpl_cmd.get_matches_mut();

    if let Some(account_matches) = matches.subcommand_matches("account") {
        if let Some(info_matches) = account_matches.subcommand_matches("info") {
            account_info(account_matches, info_matches).await?;
        } else if let Some(balance_matches) = account_matches.subcommand_matches("balances") {
            account_balances(account_matches, balance_matches).await?;
        } else if let Some(offers_matches) = account_matches.subcommand_matches("offers") {
            account_offers(account_matches, offers_matches).await?;
        } else if let Some(offers_matches) = account_matches.subcommand_matches("trustlines") {
            account_trustlines(account_matches, offers_matches).await?;
        }
    } else if let Some(ledger_matches) = matches.subcommand_matches("ledger") {
        // #TODO properly handle this
        ledger_closed(ledger_matches).await?;
    } else {
        xrpl_cmd.print_long_help().unwrap();
    }

    Ok(())
}
