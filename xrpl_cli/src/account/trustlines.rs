pub mod list_trustlines;

use clap::ArgMatches;

use self::list_trustlines::list_trustlines;

// #todo add option to cleanup empty trustlines (balance 0)
// #todo add option to remove trustline (auto return balance to issuer account)

pub async fn account_trustlines(
    account_matches: &ArgMatches,
    lines_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    if let Some(list_trustlines_matches) = lines_matches.subcommand_matches("list") {
        list_trustlines(account, list_trustlines_matches).await?;
    }

    Ok(())
}
