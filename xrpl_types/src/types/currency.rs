use crate::{AccountId, CurrencyCode, Error};

/// Currency: XRP or issued token. See <https://xrpl.org/currency-formats.html#specifying-without-amounts>
/// and currency part of <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Currency {
    Issued(IssuedCurrency),
    Xrp,
}

impl Currency {
    pub fn xrp() -> Self {
        Self::Xrp
    }

    pub fn issued(currency_code: CurrencyCode, issuer: AccountId) -> Result<Self, Error> {
        Ok(Self::Issued(IssuedCurrency::try_new(
            currency_code,
            issuer,
        )?))
    }

    pub fn is_xrp(&self) -> bool {
        *self == Currency::Xrp
    }

    pub fn is_issued(&self) -> bool {
        matches!(self, Currency::Issued(_))
    }
}

/// Issued currency. See <https://xrpl.org/currency-formats.html#specifying-without-amounts>
/// and currency part of <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct IssuedCurrency {
    // fields are private since it is validated when the IssuedAmount value is created
    currency_code: CurrencyCode,
    issuer: AccountId,
}

impl IssuedCurrency {
    pub fn try_new(currency_code: CurrencyCode, issuer: AccountId) -> Result<Self, Error> {
        if currency_code.is_xrp() {
            return Err(Error::InvalidData(
                "Issued amount cannot have XRP currency code".to_string(),
            ));
        }
        Ok(Self {
            currency_code,
            issuer,
        })
    }

    /// Currency code, see <https://xrpl.org/serialization.html#amount-fields>
    pub fn currency_code(&self) -> CurrencyCode {
        self.currency_code
    }

    /// Issuer of token, see <https://xrpl.org/serialization.html#amount-fields>
    pub fn issuer(&self) -> AccountId {
        self.issuer
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;

    #[test]
    fn test_xrp() {
        let currency = Currency::xrp();
        assert!(currency.is_xrp());
        assert!(!currency.is_issued());
    }

    #[test]
    fn test_issued() {
        let currency = Currency::issued(
            CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        )
        .unwrap();
        assert!(!currency.is_xrp());
        assert!(currency.is_issued());
    }

    /// Issued amount with XRP currency code is not valid
    #[test]
    fn test_issued_currency_xrp() {
        let result = IssuedCurrency::try_new(
            CurrencyCode::xrp(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        );
        assert_matches!(result, Err(Error::InvalidData(message)) => {
            assert!(message.contains("Issued amount cannot have XRP currency code"), "message: {}", message);
        });
    }

    /// Test we can created issued amount with standard currency code
    #[test]
    fn test_issued_currency_standard() {
        IssuedCurrency::try_new(
            CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        )
        .unwrap();
    }

    /// Test we can created issued amount with non-standard currency code
    #[test]
    fn test_issued_amount_non_standard() {
        IssuedCurrency::try_new(
            CurrencyCode::non_standard([
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            ])
            .unwrap(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        )
        .unwrap();
    }
}
