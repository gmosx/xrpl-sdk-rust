use crate::{AccountId, Error};
use ascii::{AsciiChar, AsciiStr, AsciiString};
use std::fmt::{Display, Formatter, Write};
use std::str::FromStr;

/// Currency code <https://xrpl.org/currency-formats.html#currency-codes>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CurrencyCode {
    /// Xrp special case, see <https://xrpl.org/currency-formats.html#standard-currency-codes>
    Xrp,
    /// Iso style currency code <https://xrpl.org/currency-formats.html#standard-currency-codes>
    Standard(StandardCurrencyCode),
    /// Hex style currency code <https://xrpl.org/currency-formats.html#nonstandard-currency-codes>
    NonStandard(NonStandardCurrencyCode),
}

impl CurrencyCode {
    pub fn xrp() -> Self {
        CurrencyCode::Xrp
    }

    pub fn standard(chars: [AsciiChar; 3]) -> Result<Self, Error> {
        Ok(CurrencyCode::Standard(
            StandardCurrencyCode::from_ascii_chars(chars)?,
        ))
    }

    pub fn non_standard(bytes: [u8; 20]) -> Result<Self, Error> {
        Ok(CurrencyCode::NonStandard(
            NonStandardCurrencyCode::from_bytes(bytes)?,
        ))
    }
}

impl FromStr for CurrencyCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "XRP" {
            Ok(CurrencyCode::Xrp)
        } else if s.len() == 3 {
            let ascii_chars = to_3_ascii_chars(s)?;
            CurrencyCode::standard(ascii_chars)
        } else {
            let bytes = hex::decode(s).map_err(|err| {
                Error::InvalidData(format!(
                    "Currency code is neither three letter symbol neither hex string: {}",
                    s
                ))
            })?;
            let bytes: [u8; 20] = bytes.try_into().map_err(|_| {
                Error::InvalidData("Currency code hex string is not 20 bytes".to_string())
            })?;
            CurrencyCode::non_standard(bytes)
        }
    }
}

impl Display for CurrencyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyCode::Xrp => f.write_str("XRP"),
            CurrencyCode::Standard(code) => code.fmt(f),
            CurrencyCode::NonStandard(code) => code.fmt(f),
        }
    }
}

/// Iso style currency code <https://xrpl.org/currency-formats.html#standard-currency-codes>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
// tuple is private since it is validated when the NonStandardCurrencyCode value is created
pub struct StandardCurrencyCode([AsciiChar; 3]);

impl StandardCurrencyCode {
    pub fn from_ascii_chars(chars: [AsciiChar; 3]) -> Result<Self, Error> {
        if chars == [AsciiChar::X, AsciiChar::R, AsciiChar::P] {
            return Err(Error::InvalidData(
                "XRP is not a valid standard currency code".to_string(),
            ));
        }
        Ok(Self(chars))
    }

    pub fn as_bytes(&self) -> [u8; 3] {
        *<&[u8; 3]>::try_from(self.as_ascii_str().as_bytes()).expect("has length 3")
    }

    pub fn as_ascii_chars(&self) -> [AsciiChar; 3] {
        self.0
    }

    pub fn as_str(&self) -> &str {
        self.as_ascii_str().as_str()
    }

    pub fn as_ascii_str(&self) -> &AsciiStr {
        <&AsciiStr>::from(&self.0 as &[AsciiChar])
    }
}

impl AsRef<[u8]> for StandardCurrencyCode {
    fn as_ref(&self) -> &[u8] {
        self.as_ascii_str().as_bytes()
    }
}

impl AsRef<str> for StandardCurrencyCode {
    fn as_ref(&self) -> &str {
        self.as_ascii_str().as_str()
    }
}

impl Display for StandardCurrencyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_ascii_str().fmt(f)
    }
}

/// Hex style currency code <https://xrpl.org/currency-formats.html#nonstandard-currency-codes>
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
// tuple is private since it is validated when the NonStandardCurrencyCode value is created
pub struct NonStandardCurrencyCode([u8; 20]);

impl NonStandardCurrencyCode {
    pub fn from_bytes(bytes: [u8; 20]) -> Result<Self, Error> {
        if bytes[0] == 0x00 {
            return Err(Error::InvalidData(
                "Non-standard Currency code must start with byte of value zero".to_string(),
            ));
        }
        Ok(Self(bytes))
    }

    pub fn as_bytes(&self) -> &[u8; 20] {
        &self.0
    }
}

impl AsRef<[u8]> for NonStandardCurrencyCode {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

impl Display for NonStandardCurrencyCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&hex::encode_upper(self.as_bytes()))
    }
}

fn to_3_ascii_chars(str: &str) -> Result<[AsciiChar; 3], Error> {
    let ascii_string = AsciiString::from_str(str)
        .map_err(|err| Error::InvalidData(format!("Not valid ascii string: {}", err)))?;
    let ascii_chars = <&[AsciiChar; 3]>::try_from(ascii_string.as_slice())
        .map_err(|err| Error::InvalidData(format!("String does not have length 3: {}", err)))?;
    Ok(*ascii_chars)
}

#[cfg(test)]
mod test {
    use super::*;
    use ascii::{AsAsciiStr, AsciiString};
    use assert_matches::assert_matches;
    use std::str::FromStr;

    #[test]
    fn test_non_standard_code_from_bytes() {
        let code = NonStandardCurrencyCode::from_bytes([
            0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ])
        .unwrap();
        assert_eq!(
            code.as_bytes(),
            &[
                0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x02
            ]
        );
    }

    /// Code is invalid if first byte is 0x00, see <https://xrpl.org/currency-formats.html#nonstandard-currency-codes>
    #[test]
    fn test_non_standard_code_from_bytes_invalid_code() {
        let result = NonStandardCurrencyCode::from_bytes([
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x02,
        ]);
        assert_matches!(result, Err(Error::InvalidData(message)) => {
            assert!(message.contains("Non-standard Currency code must start with byte of value zero"), "message: {}", message);
        });
    }

    #[test]
    fn test_standard_code_from_ascii_chars() {
        let code =
            StandardCurrencyCode::from_ascii_chars(to_3_ascii_chars("USD").unwrap()).unwrap();
        assert_eq!(code.as_str(), "USD");
        assert_eq!(
            code.as_ascii_str(),
            &[AsciiChar::U, AsciiChar::S, AsciiChar::D] as &[AsciiChar]
        );
        assert_eq!(code.as_bytes(), [b'U', b'S', b'D']);
        assert_eq!(
            code.as_ascii_chars(),
            [AsciiChar::U, AsciiChar::S, AsciiChar::D]
        );
    }

    /// Test creating standard currency code from the chars XRP. This should fail since it is not
    /// allowed, see <https://xrpl.org/currency-formats.html#standard-currency-codes>
    #[test]
    fn test_standard_code_from_ascii_chars_xrp() {
        let result = StandardCurrencyCode::from_ascii_chars(to_3_ascii_chars("XRP").unwrap());
        assert_matches!(result, Err(Error::InvalidData(message)) => {
            assert!(message.contains("XRP is not a valid standard currency code"), "message: {}", message);
        });
    }

    /// Test parsing standard currency code from string and converting back to string
    #[test]
    fn test_parse_xrp_currency_code() {
        let code = CurrencyCode::from_str("XRP").unwrap();
        assert_matches!(code, CurrencyCode::Xrp);
        assert_eq!(code.to_string(), "XRP");
    }

    /// Test parsing standard currency code from string and converting back to string
    #[test]
    fn test_parse_standard_currency_code() {
        let code = CurrencyCode::from_str("USD").unwrap();
        assert_matches!(
            code,
            CurrencyCode::Standard(StandardCurrencyCode([
                AsciiChar::U,
                AsciiChar::S,
                AsciiChar::D
            ]))
        );
        assert_eq!(code.to_string(), "USD");
    }

    /// Test parsing standard currency code from string and converting back to string
    #[test]
    fn test_parse_non_standard_currency_code() {
        let code = CurrencyCode::from_str("434F524500000000000000000000000000000000").unwrap();
        assert_matches!(
            code,
            CurrencyCode::NonStandard(NonStandardCurrencyCode([
                0x43, 0x4F, 0x52, 0x45, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00
            ]))
        );
        assert_eq!(code.to_string(), "434F524500000000000000000000000000000000");
    }
}
