use clap::ArgMatches;

use xrpl_api::types::OfferCancelTransaction;
use xrpl_sdk_jsonrpc::Client;

pub fn remove_offer(account: impl Into<String>, remove_offer_matches: &ArgMatches) {
    let account = account.into();

    let offer_sequence: &String = remove_offer_matches
        .get_one("OFFER_SEQ")
        .expect("offer sequence missing");
    let offer_sequence: u32 = offer_sequence.parse().expect("offer sequence invalid");

    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async {
        let _client = Client::new();
        let tx = OfferCancelTransaction::new(account, offer_sequence);
        dbg!(tx);
        // #todo submit the transaction.
    });
}
