pub mod list_offers;
pub mod offer_info;
pub mod remove_offer;

use clap::ArgMatches;

use self::list_offers::list_offers;
use self::remove_offer::remove_offer;

pub async fn account_offers(
    account_matches: &ArgMatches,
    offers_matches: &ArgMatches,
) -> anyhow::Result<()> {
    let account: &String = account_matches.get_one("ACCOUNT").unwrap();

    if let Some(remove_offer_matches) = offers_matches.subcommand_matches("remove") {
        let public_key: &String = account_matches.get_one("PUBLIC_KEY").unwrap();
        let secret_key: &String = account_matches.get_one("SECRET_KEY").unwrap();
        remove_offer(account, public_key, secret_key, remove_offer_matches).await?;
    } else if let Some(list_offers_matches) = offers_matches.subcommand_matches("list") {
        list_offers(account, list_offers_matches).await?;
    }

    Ok(())
}
