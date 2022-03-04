use xrpl_types::Transaction;

use super::serializer::Serializer;

const MIN_IOU_EXPONENT: i64 = -96;
const MAX_IOU_EXPONENT: i64 = 80;
// const MAX_IOU_PRECISION: i64 = 16;
const MIN_MANTISSA: i64 = 10i64.pow(15);
const MAX_MANTISSA: i64 = 10i64.pow(16) - 1;

/// Converts a string to the internal binary format. Please note, this is a decimal
/// format, not a floating-point one.
///
/// - https://github.com/ripple/rippled/blob/develop/src/ripple/protocol/impl/STAmount.cpp#L781
/// - https://github.com/XRPLF/xrpl-py/blob/master/xrpl/core/binarycodec/types/amount.py#L141
pub fn internal_number_from_string(s: &str) -> u64 {
    // TODO: handle sign
    // TODO: handle zero case
    // TODO: handle integer case
    // TODO: handle unwraps!

    if s == "0" || s == "0.0" {
        // Special case for zero value.
        return 0b1000000000000000000000000000000000000000000000000000000000000000;
    }

    let mut mantissa: i64;
    let mut exponent: i64;

    if s.contains('.') {
        let (integer, decimal) = s.split_once('.').unwrap();

        mantissa = format!("{}{}", integer, decimal).parse().unwrap();
        exponent = -(decimal.len() as i64);
    } else {
        mantissa = s.parse().unwrap();
        exponent = 0;
    }

    // let (integer, decimal) = s.split_once('.').unwrap();

    // let mut mantissa: i64 = format!("{}{}", integer, decimal).parse().unwrap();
    // let mut exponent: i64 = -(decimal.len() as i64);

    // Normalize to expected range.

    while mantissa < MIN_MANTISSA && exponent > MIN_IOU_EXPONENT {
        mantissa *= 10;
        exponent -= 1;
    }

    while mantissa > MAX_MANTISSA {
        if exponent < MAX_IOU_EXPONENT {
            mantissa /= 10;
            exponent += 1;
        }
    }

    if exponent < MIN_IOU_EXPONENT || mantissa < MIN_MANTISSA {
        // Round to zero.
        return 0b1000000000000000000000000000000000000000000000000000000000000000;
    }

    // TODO:
    //
    // if exp > _MAX_IOU_EXPONENT or mantissa > _MAX_MANTISSA:
    //     raise XRPLBinaryCodecException(
    //         f"Amount overflow in issued currency value {str(value)}"
    //     )

    // bit 63: 1 = not XRP
    // bit 62: 1 = positive
    let mask = 0b1100000000000000000000000000000000000000000000000000000000000000;

    let mantissa = mantissa as u64;
    let exponent = (97 + exponent) as u64;

    mantissa | (exponent << 54) | mask
}

pub fn serialize_transaction(tx: &Transaction) -> Vec<u8> {
    let mut s = Serializer::new();
    s.push_transaction(tx, None);
    s.to_vec()
}

pub fn serialize_transaction_to_hex(tx: &Transaction) -> String {
    let mut s = Serializer::new();
    s.push_transaction(tx, None);
    hex::encode(s.to_vec()).to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_internal_number_from_string() {
        let n = internal_number_from_string("1200.34");
        // println!("{:b}", n);
        assert_eq!(
            n,
            0b1101010101000100010000111011001111101111010011110100100000000000
        );

        // Integer
        let n = internal_number_from_string("1200");
        // println!("{:b}", n);
        assert_eq!(
            n,
            0b1101010101000100010000110110010011000101101110110000000000000000
        );
    }
}
