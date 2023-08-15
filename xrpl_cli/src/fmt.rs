use xrpl_sdk_jsonrpc::AccountOffer;
use xrpl_types::{Amount, IssuedAmount};

pub fn xrp_to_drops(xrp: f64) -> u64 {
    (xrp * 1_000_000.0) as u64
}

pub fn drops_to_xrp(drops: u64) -> f64 {
    (drops as f64) / 1_000_000.0
}

pub fn format_amount(amount: &Amount) -> String {
    match amount {
        Amount::Issued(IssuedAmount {
            value,
            currency,
            issuer,
        }) => {
            format!("{value} {currency}.{issuer}")
        }
        Amount::Drops(drops) => {
            format!("{} XRP", drops_to_xrp(drops.parse().unwrap_or_default()))
        }
    }
}

pub fn format_offer(offer: &AccountOffer) -> String {
    format!(
        "{}: taker-pays='{}', taker-gets='{}'",
        offer.seq,
        format_amount(&offer.taker_pays),
        format_amount(&offer.taker_gets)
    )
}
