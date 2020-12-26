use std::char::from_digit;
use std::convert::{TryFrom, TryInto};
use std::iter::{once, repeat};

/// Encode a property of a slice of data with finite cardinality into a single number
/// There must be a mapping from this property to a digit in a certain base
/// The output is bound by base^(data.len() - 1)
/// Precondition: the property being encoded must satisfy the parity property, which is
/// to say that summing all the values of the property should give a multiple of the
/// base. This is used to omit one "bit" in that base as it can be reconstituted later using
/// parity.
pub fn encode_property<T: Ord, Encoded: TryFrom<usize>>(
    data: &[T],
    property_mapping: &dyn Fn(&T) -> u8,
    base: u8,
) -> Result<Encoded, Encoded::Error> {
    let n = data.len();

    // n - 1 "bits" of information in a given base are all we need
    // The last "bit" doesn't need to be stored as the sum of all bits must be divisible by the base
    let bits_string = data
        .iter()
        .take(n - 1)
        .map(property_mapping)
        .map(|digit| from_digit(digit as u32, base as u32).unwrap())
        .collect::<String>();

    usize::from_str_radix(&bits_string, base as u32)
        .expect("The orientation1 should be convertible to the correct radix")
        .try_into()
}

/// Decode a property number into a unique ordering of this slice's property.
///
/// Take as input the property number produced by `encode_property` and the base that was used to
/// encode it.
/// Returns a Vec<u8> filled with the value for the property at each position.
pub fn decode_property<ToDecode: TryInto<usize>>(
    property: ToDecode,
    base: u8,
    len: usize,
) -> Result<Vec<u8>, ToDecode::Error> {
    let property = property.try_into()?;
    let bits_string = format!("{}", radix_fmt::radix(property, base));
    let last_digit = (base as u32
        - bits_string
            .chars()
            .map(|c| c.to_digit(base as u32).unwrap())
            .sum::<u32>()
            % base as u32) as u8;
    let ret = repeat(0)
        .take(len - 1 - bits_string.len())
        .chain(
            bits_string
                .chars()
                .map(|c| c.to_digit(base as u32).unwrap() as u8),
        )
        .chain(once(last_digit))
        .collect();
    Ok(ret)
}

#[cfg(test)]
mod tests {
    use super::*;
    const SEQ: [u8; 8] = [3, 6, 5, 7, 0, 2, 1, 4];
    const PROP: [u8; 8] = [2, 0, 0, 1, 1, 0, 0, 2];
    fn mapping(x: &u8) -> u8 {
        match *x {
            3 | 4 => 2,
            7 | 0 => 1,
            6 | 5 | 2 | 1 => 0,
            _ => panic!("should only be called with values from seq"),
        }
    }
    #[test]
    fn test_encode_property() {
        // 2*3^6 + 0*3^5 + 0*3^4 + 1*3^3 +1*3^2 + 0*3^1 + 0*3^0 = 1494
        // The last ternary bit is not encoded as it can be reconstituted by parity:
        // the sum of all terms must be a multiple of the radix
        assert_eq!(Ok(1494), encode_property(&SEQ, &mapping, 3));
    }
    #[test]
    fn test_encode_property_fails_with_too_small_type() {
        assert!(encode_property::<_, u8>(&SEQ, &mapping, 3).is_err());
    }
    #[test]
    fn test_decode_property() {
        assert_eq!(Ok(PROP.to_vec()), decode_property(1494, 3, 8));
    }
}
