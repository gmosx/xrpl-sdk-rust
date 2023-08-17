use xrpl_http_client::AccountOffer;
use xrpl_types::{Amount, IssuedAmount};

// #todo extract some of these functions to another crate, to promote reuse.

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

/// Tries to construct an amount from a 'spec' string.
///
/// Examples:
/// Amount::try_from_str("26.231 USD.rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq");
/// Amount::try_from_str("11.1 XRP");
pub fn amount_from_str(s: impl AsRef<str>) -> Option<Amount> {
    let mut parts = s.as_ref().split_whitespace();

    let Some(value) = parts.next() else {
            return None;
        };

    let Some(currency) = parts.next() else {
            return None;
        };

    if currency.to_uppercase() == "XRP" {
        return Some(Amount::xrp(value));
    }

    let mut currency_parts = currency.split(".");

    let Some(currency) = currency_parts.next() else {
            return None;
        };

    let Some(issuer) = currency_parts.next() else {
            return None;
        };

    Some(Amount::issued(value, currency.to_uppercase(), issuer))
}

#[cfg(test)]
mod test {
    use super::*;
    use assert_matches::assert_matches;

    #[test]
    fn amount_from_spec_string() {
        let amount = amount_from_str("26.231 USD.rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq").unwrap();
        assert_matches!(
            amount,
            Amount::Issued(IssuedAmount {
                value,
                currency,
                issuer
            }) if value == "26.231" && currency == "USD" && issuer == "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq"
        );

        let amount = amount_from_str("11.1 XRP").unwrap();
        assert_matches!(
            amount,
            Amount::Drops(drops) if drops == "11100000"
        );
    }
}
