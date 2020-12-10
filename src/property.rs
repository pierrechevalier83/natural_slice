use std::char::from_digit;
use std::iter::once;

/// Encode a property of a slice of data with finite cardinality into a singl number
/// radix ^ data.len()
pub fn encode_property<T: Ord>(
    data: &[T],
    property_mapping: &dyn Fn(&T) -> u8,
    radix: u8,
) -> usize {
    let n = data.len();

    // n - 1 "bits" of information in a given radix are all we need
    // The last "bit" can be obtained as the sum of all bits must be divisible by the radix
    let bits_string = data
        .iter()
        .take(n - 1)
        .map(property_mapping)
        .map(|digit| from_digit(digit as u32, radix as u32).unwrap())
        .collect::<String>();

    usize::from_str_radix(&bits_string, radix as u32)
        .expect("The orientation1 should be convertible to the correct radix")
}

pub fn decode_property(property: usize, radix: u8) -> Vec<u8> {
    let bits_string = format!("{}", radix_fmt::radix(property, radix));
    let last_digit = radix
        - bits_string
            .chars()
            .map(|c| c.to_digit(radix as u32).unwrap() as u8)
            .sum::<u8>()
            % radix;
    bits_string
        .chars()
        .map(|c| c.to_digit(radix as u32).unwrap() as u8)
        .chain(once(last_digit))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_encode_property() {
        let seq = [3, 6, 5, 7, 0, 2, 1, 4];
        //         2  0  0  1  1  0  0  2
        let mapping = |x: &u8| match *x {
            3 | 4 => 2,
            7 | 0 => 1,
            6 | 5 | 2 | 1 => 0,
            _ => panic!("should only be called with values from seq"),
        };
        // 2*3^6 + 0*3^5 + 0*3^4 + 1*3^3 +1*3^2 + 0*3^1 + 0*3^0 = 1494
        // The last ternary bit is not encoded as it can be reconstituted by parity:
        // the sum of all terms must be a multiple of the radix
        assert_eq!(1494, encode_property(&seq, &mapping, 3));
    }
    #[test]
    fn test_decode_property() {
        assert_eq!(vec![2, 0, 0, 1, 1, 0, 0, 2], decode_property(1494, 3));
    }
}
