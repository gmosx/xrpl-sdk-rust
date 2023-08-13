use clap::ArgMatches;

pub fn remove_offer(account_id: &str, remove_offer_matches: &ArgMatches) {
    let offer_id: &String = remove_offer_matches.get_one("OFFER_ID").unwrap();

    println!("-- hello -- {account_id} : {offer_id}");
}
