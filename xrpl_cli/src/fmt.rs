use rust_decimal::Decimal;
use std::str::FromStr;
use xrpl_http_client::AccountOffer;
use xrpl_types::{AccountId, IssuedValue};

// #todo extract some of these functions to another crate, to promote reuse.

pub fn xrp_to_drops(xrp: f64) -> u64 {
    (xrp * 1_000_000.0) as u64
}

pub fn drops_to_xrp(drops: u64) -> f64 {
    (drops as f64) / 1_000_000.0
}

pub fn format_amount(amount: &xrpl_api::Amount) -> String {
    match amount {
        xrpl_api::Amount::Issued(xrpl_api::IssuedAmount {
            value,
            currency,
            issuer,
        }) => {
            format!("{value} {currency}.{issuer}")
        }
        xrpl_api::Amount::Drops(drops) => {
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
pub fn amount_from_str(s: impl AsRef<str>) -> Option<xrpl_types::Amount> {
    let mut parts = s.as_ref().split_whitespace();

    let Some(value) = parts.next() else {
            return None;
        };

    let Some(currency) = parts.next() else {
            return None;
        };

    if currency.to_uppercase() == "XRP" {
        return xrpl_types::Amount::drops((f64::from_str(value).ok()? * 1_000_000.0) as u64).ok();
    }

    let mut currency_parts = currency.split('.');

    let Some(currency) = currency_parts.next() else {
            return None;
        };

    let Some(issuer) = currency_parts.next() else {
            return None;
        };

    let value = Decimal::from_str(value).ok()?;
    let issued_value = IssuedValue::from_mantissa_exponent(
        value.mantissa().try_into().ok()?,
        -value.scale().try_into().ok()?,
    )
    .ok()?;
    let currency_code = xrpl_types::CurrencyCode::from_str(&currency.to_uppercase()).ok()?;
    let issuer = AccountId::from_address(issuer).ok()?;

    xrpl_types::Amount::issued(issued_value, currency_code, issuer).ok()
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
            xrpl_types::Amount::Issued(issued_amount) => {
                assert_eq!(issued_amount.value().mantissa(), 2623100000000000);
                assert_eq!(issued_amount.value().exponent(), -14);
                assert_eq!(issued_amount.currency().to_string(), "USD");
                assert_eq!(issued_amount.issuer().to_address(), "rhub8VRN55s94qWKDv6jmDy1pUykJzF3wq");
            }
        );

        let amount = amount_from_str("11.1 XRP").unwrap();
        assert_matches!(
            amount,
            xrpl_types::Amount::Drops(drops) => {
                assert_eq!(drops.drops(), 11100000);
            }
        );
    }
}
