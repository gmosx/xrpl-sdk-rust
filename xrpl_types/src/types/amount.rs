use crate::{AccountId, CurrencyCode, Error};

/// Amount of XRP or issued token. See <https://xrpl.org/currency-formats.html#specifying-currency-amounts>
/// and <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Amount {
    Issued(IssuedAmount),
    Drops(DropsAmount),
}

impl Amount {
    pub fn drops(drops: u64) -> Result<Self, Error> {
        Ok(Self::Drops(DropsAmount::from_drops(drops)?))
    }

    pub fn issued(
        value: IssuedValue,
        currency: CurrencyCode,
        issuer: AccountId,
    ) -> Result<Self, Error> {
        Ok(Self::Issued(IssuedAmount::from_issued_value(
            value, currency, issuer,
        )?))
    }

    pub fn is_drops(&self) -> bool {
        matches!(self, Amount::Drops(_))
    }

    pub fn is_issued(&self) -> bool {
        matches!(self, Amount::Issued(_))
    }
}

/// Amount of XRP in drops, see <https://xrpl.org/currency-formats.html#xrp-amounts>
/// and <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
// tuple is private since it is validated when the DropsAmount value is created
pub struct DropsAmount(u64);

impl DropsAmount {
    pub fn from_drops(drops: u64) -> Result<Self, Error> {
        if drops & (0b11 << 62) != 0 {
            return Err(Error::OutOfRange(
                "Drop amounts cannot use the two must significant bits".to_string(),
            ));
        }
        Ok(Self(drops))
    }

    /// Amount of XRP in drops
    pub fn drops(&self) -> u64 {
        self.0
    }
}

/// Amount of issued token. See <https://xrpl.org/currency-formats.html#token-amounts>
/// and <https://xrpl.org/serialization.html#amount-fields>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct IssuedAmount {
    // fields are private since it is validated when the IssuedAmount value is created
    value: IssuedValue,
    currency: CurrencyCode,
    issuer: AccountId,
}

impl IssuedAmount {
    pub fn from_issued_value(
        value: IssuedValue,
        currency: CurrencyCode,
        issuer: AccountId,
    ) -> Result<Self, Error> {
        if currency.is_xrp() {
            return Err(Error::InvalidData(
                "Issued amount cannot have XRP currency code".to_string(),
            ));
        }
        Ok(Self {
            value,
            currency,
            issuer,
        })
    }

    /// Decimal representation of token amount, see <https://xrpl.org/serialization.html#amount-fields>
    pub fn value(&self) -> IssuedValue {
        self.value
    }

    /// Currency code, see <https://xrpl.org/serialization.html#amount-fields>
    pub fn currency(&self) -> CurrencyCode {
        self.currency
    }

    /// Issuer of token, see <https://xrpl.org/serialization.html#amount-fields>
    pub fn issuer(&self) -> AccountId {
        self.issuer
    }
}

/// The value of issued amount, see <https://xrpl.org/serialization.html#token-amount-format>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct IssuedValue {
    // fields are private since it is validated when the IssuedValue value is created
    mantissa: i64,
    exponent: i8,
}

impl IssuedValue {
    /// Creates value from given mantissa and exponent. The created value will be normalized
    /// according to <https://xrpl.org/serialization.html#token-amount-format>. If the value
    /// cannot be represented, an error is returned.
    pub fn from_mantissa_exponent(mantissa: i64, exponent: i8) -> Result<Self, Error> {
        Self { mantissa, exponent }.normalize()
    }

    /// The value zero
    pub fn zero() -> Self {
        Self {
            mantissa: 0,
            exponent: 0,
        }
    }

    /// Signed and normalized mantissa, see <https://xrpl.org/serialization.html#token-amount-format>
    pub fn mantissa(&self) -> i64 {
        self.mantissa
    }

    /// Normalized exponent, see <https://xrpl.org/serialization.html#token-amount-format>
    pub fn exponent(&self) -> i8 {
        self.exponent
    }

    fn normalize(self) -> Result<Self, Error> {
        const MANTISSA_MIN: i64 = 1000000000000000;
        const MANTISSA_MAX: i64 = 9999999999999999;
        const EXPONENT_MIN: i8 = -96;
        const EXPONENT_MAX: i8 = 80;

        let mut exponent = self.exponent;
        let (mut mantissa, negative) = match self.mantissa {
            0 => {
                return Ok(Self::zero());
            }
            1.. => (self.mantissa, false),
            ..=-1 => (
                self.mantissa.checked_neg().ok_or_else(|| {
                    Error::OutOfRange("Specified mantissa cannot be i64::MIN".to_string())
                })?,
                true,
            ),
        };

        while mantissa < MANTISSA_MIN && exponent > EXPONENT_MIN {
            mantissa *= 10;
            exponent -= 1;
        }

        while mantissa > MANTISSA_MAX && exponent < EXPONENT_MAX {
            mantissa /= 10;
            exponent += 1;
        }

        if mantissa > MANTISSA_MAX || exponent > EXPONENT_MAX {
            return Err(Error::OutOfRange(format!(
                "Issued value too big to be normalized: {:?}",
                self
            )));
        }

        if mantissa < MANTISSA_MIN || exponent < EXPONENT_MIN {
            return Ok(Self::zero());
        }

        if negative {
            mantissa = -mantissa;
        }

        Ok(Self { mantissa, exponent })
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ascii::AsciiChar;
    use assert_matches::assert_matches;

    #[test]
    fn test_drops_amount() {
        let amount = DropsAmount::from_drops(0).unwrap();
        assert_eq!(amount.drops(), 0);
        let amount = DropsAmount::from_drops(10000).unwrap();
        assert_eq!(amount.drops(), 10000);
        // test max value
        let amount = DropsAmount::from_drops(u64::MAX >> 2).unwrap();
        assert_eq!(amount.drops(), u64::MAX >> 2);
    }

    /// We cannot use the two first bits, those values are out of range
    /// <https://xrpl.org/serialization.html#amount-fields>
    #[test]
    fn test_drops_amount_out_of_range() {
        let result = DropsAmount::from_drops(1 << 62);
        assert_matches!(result, Err(Error::OutOfRange(message)) => {
            assert!(message.contains("Drop amounts cannot use the two must significant bits"), "message: {}", message);
        });
        let result = DropsAmount::from_drops(1 << 63);
        assert_matches!(result, Err(Error::OutOfRange(message)) => {
            assert!(message.contains("Drop amounts cannot use the two must significant bits"), "message: {}", message);
        });
    }

    /// Issued amount with XRP currency code is not valid
    #[test]
    fn test_issued_amount_xrp() {
        let result = IssuedAmount::from_issued_value(
            IssuedValue::from_mantissa_exponent(1, 0).unwrap(),
            CurrencyCode::xrp(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        );
        assert_matches!(result, Err(Error::InvalidData(message)) => {
            assert!(message.contains("Issued amount cannot have XRP currency code"), "message: {}", message);
        });
    }

    /// Test we can created issued amount with standard currency code
    #[test]
    fn test_issued_amount_standard() {
        IssuedAmount::from_issued_value(
            IssuedValue::from_mantissa_exponent(1, 0).unwrap(),
            CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        )
        .unwrap();
    }

    /// Test we can created issued amount with non-standard currency code
    #[test]
    fn test_issued_amount_non_standard() {
        IssuedAmount::from_issued_value(
            IssuedValue::from_mantissa_exponent(1, 0).unwrap(),
            CurrencyCode::non_standard([
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
            ])
            .unwrap(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        )
        .unwrap();
    }

    #[test]
    fn test_issued_value_zero() {
        let value = IssuedValue::from_mantissa_exponent(0, 0).unwrap();
        assert_eq!(value.mantissa(), 0);
        assert_eq!(value.exponent(), 0);
    }

    #[test]
    fn test_issued_value_one() {
        let value = IssuedValue::from_mantissa_exponent(1, 0).unwrap();
        assert_eq!(value.mantissa(), 1_000_000_000_000_000);
        assert_eq!(value.exponent(), -15);
    }

    #[test]
    fn test_issued_value_minus_one() {
        let value = IssuedValue::from_mantissa_exponent(-1, 0).unwrap();
        assert_eq!(value.mantissa(), -1_000_000_000_000_000);
        assert_eq!(value.exponent(), -15);
    }

    /// Exponent is always zero for the value zero
    #[test]
    fn test_issued_value_zero_normalize_exponent() {
        let value = IssuedValue::from_mantissa_exponent(0, 10).unwrap();
        assert_eq!(value.mantissa(), 0);
        assert_eq!(value.exponent(), 0);
    }

    /// Test mantissa is scaled up to the normalized range
    #[test]
    fn test_issued_value_scale_up() {
        let value = IssuedValue::from_mantissa_exponent(123, 0).unwrap();
        assert_eq!(value.mantissa(), 1_230_000_000_000_000);
        assert_eq!(value.exponent(), -13);
    }

    /// Test mantissa is scaled down to the normalized range
    #[test]
    fn test_issued_value_scale_down() {
        let value = IssuedValue::from_mantissa_exponent(1_230_000_000_000_000_000, 0).unwrap();
        assert_eq!(value.mantissa(), 1_230_000_000_000_000);
        assert_eq!(value.exponent(), 3);
    }

    /// Test negative value
    #[test]
    fn test_issued_value_negative() {
        let value = IssuedValue::from_mantissa_exponent(-123, 0).unwrap();
        assert_eq!(value.mantissa(), -1_230_000_000_000_000);
        assert_eq!(value.exponent(), -13);
    }

    /// Test hitting the mantissa min value when scaling up
    #[test]
    fn test_issued_value_mantissa_min_scale_up() {
        let value = IssuedValue::from_mantissa_exponent(1, 0).unwrap();
        assert_eq!(value.mantissa(), 1_000_000_000_000_000);
        assert_eq!(value.exponent(), -15);
    }

    /// Test hitting the mantissa min value when scaling down
    #[test]
    fn test_issued_value_mantissa_min_scale_down() {
        let value = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000_000, 0).unwrap();
        assert_eq!(value.mantissa(), 1_000_000_000_000_000);
        assert_eq!(value.exponent(), 3);
    }

    /// Test hitting the mantissa max value when scaling down
    #[test]
    fn test_issued_value_mantissa_max_scale_down() {
        let value = IssuedValue::from_mantissa_exponent(999_999_999_999_999_900, 0).unwrap();
        assert_eq!(value.mantissa(), 9_999_999_999_999_999);
        assert_eq!(value.exponent(), 2);
    }

    /// Test hitting exponent max value when scaling down mantissa
    #[test]
    fn test_issued_value_exponent_max() {
        let value = IssuedValue::from_mantissa_exponent(1_230_000_000_000_000_000, 77).unwrap();
        assert_eq!(value.mantissa(), 1_230_000_000_000_000);
        assert_eq!(value.exponent(), 80);
    }

    /// Test going over exponent max value when scaling down mantissa
    #[test]
    fn test_issued_value_out_of_range_too_big_mantissa() {
        let result = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000_000, 78);
        assert_matches!(result, Err(Error::OutOfRange(message)) => {
            assert!(message.contains("Issued value too big to be normalized"), "message: {}", message);
        });
    }

    /// Test going over exponent max value
    #[test]
    fn test_issued_value_out_of_range_too_big_exponent() {
        let result = IssuedValue::from_mantissa_exponent(1_000_000_000_000_000, 81);
        assert_matches!(result, Err(Error::OutOfRange(message)) => {
            assert!(message.contains("Issued value too big to be normalized"), "message: {}", message);
        });
    }

    /// Test hitting exponent min value when scaling up mantissa
    #[test]
    fn test_issued_value_exponent_min() {
        let value = IssuedValue::from_mantissa_exponent(123_000_000_000, -92).unwrap();
        assert_eq!(value.mantissa(), 1_230_000_000_000_000);
        assert_eq!(value.exponent(), -96);
    }

    /// Test going under exponent min value when scaling up mantissa. This
    /// should result in value zero
    #[test]
    fn test_issued_value_non_zero_normalized_to_zero_mantissa_too_small() {
        let value = IssuedValue::from_mantissa_exponent(123_000_000_000, -93).unwrap();
        assert_eq!(value.mantissa(), 0);
        assert_eq!(value.exponent(), 0);
    }

    /// Test going under exponent min value when scaling up mantissa. This
    /// should result in value zero
    #[test]
    fn test_issued_value_non_zero_normalized_to_zero_exponent_too_small() {
        let value = IssuedValue::from_mantissa_exponent(1_230_000_000_000_000, -97).unwrap();
        assert_eq!(value.mantissa(), 0);
        assert_eq!(value.exponent(), 0);
    }

    /// Test mantissa value that is `i64::MIN`
    #[test]
    fn test_issued_value_mantissa_i64_min() {
        let result = IssuedValue::from_mantissa_exponent(i64::MIN, 0);
        assert_matches!(result, Err(Error::OutOfRange(message)) => {
            assert!(message.contains("Specified mantissa cannot be i64::MIN"), "message: {}", message);
        });
    }

    #[test]
    fn test_amount_drops() {
        let amount = Amount::drops(0).unwrap();
        assert!(amount.is_drops());
    }

    #[test]
    fn test_amount_issued() {
        let amount = Amount::issued(
            IssuedValue::from_mantissa_exponent(1, 0).unwrap(),
            CurrencyCode::standard([AsciiChar::U, AsciiChar::S, AsciiChar::D]).unwrap(),
            AccountId::from_address("rf1BiGeXwwQoi8Z2ueFYTEXSwuJYfV2Jpn").unwrap(),
        )
        .unwrap();
        assert!(amount.is_issued());
    }
}
