use num_integer::binomial;
use std::convert::{TryFrom, TryInto};

/// Encode the position of the interesting elements in a slice as a single natural number.
/// Note that we don't care about the relative positions of the interesting elements, which allows
/// us to deal with a smaller number of possibilities.
///
/// In the context of a Rubiks Cube, the calculation is explained with an example here:
/// http://kociemba.org/math/UDSliceCoord.htm
pub fn encode_position<T: Ord, Encoded: TryFrom<usize>>(
    data: &[T],
    is_interesting: &dyn Fn(&T) -> bool,
) -> Result<Encoded, Encoded::Error> {
    let mut interesting_to_the_left = 0;
    data.iter()
        .enumerate()
        .map(|(index, x)| {
            if is_interesting(x) {
                interesting_to_the_left += 1;
            }
            (index, x, interesting_to_the_left)
        })
        .filter_map(|(index, x, interesting_to_the_left)| {
            if is_interesting(x) || interesting_to_the_left == 0 {
                None
            } else {
                Some((index, interesting_to_the_left))
            }
        })
        .map(|(index, interesting_to_the_left)| binomial(index, interesting_to_the_left - 1))
        .sum::<usize>()
        .try_into()
}

/// Decode the position number of a slice
/// Returns a Vec<bool> filled with false for all uninteresting elements and true for all
/// interesting elements.
/// This Vec can be used as a mapping of indices to interesting elements.
pub fn decode_position<ToDecode: TryInto<usize>>(
    position: ToDecode,
    num_interesting: usize,
    len: usize,
) -> Result<Vec<bool>, ToDecode::Error> {
    let mut interesting_to_the_left = num_interesting;
    let mut position = position.try_into()?;
    let mut result = (0..len)
        .rev()
        .map(|index| {
            let cutoff = if interesting_to_the_left > 0 {
                binomial(index, interesting_to_the_left - 1).into()
            } else {
                0
            };
            if position < cutoff {
                interesting_to_the_left -= 1;
                true
            } else {
                position -= cutoff;
                false
            }
        })
        .collect::<Vec<_>>();
    result.reverse();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn is_interesting(x: &u8) -> bool {
        *x != 0
    }
    fn examples() -> Vec<([u8; 12], usize)> {
        vec![
            ([0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1], 0),
            ([0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 1, 1], 1),
            ([0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 1, 1], 2),
            ([1, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1], 8),
            ([0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1], 9),
            ([0, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 1], 62),
            ([1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 1], 164),
            ([0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0], 165),
            ([0, 1, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0], 305),
            ([1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], 494),
        ]
    }
    #[test]
    fn test_encode_position() {
        for (data, position) in examples().iter() {
            assert_eq!(Ok(*position), encode_position(data, &is_interesting));
        }
    }
    #[test]
    fn test_encode_position_fails_with_too_small_type() {
        assert!(
            encode_position::<_, u8>(&[1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0], &is_interesting)
                .is_err()
        )
    }
    #[test]
    fn test_decode_position() {
        for (data, position) in examples().iter() {
            let expected = data.iter().map(is_interesting).collect::<Vec<_>>();
            let num_interesting = expected.iter().filter(|x| **x).count();
            assert_eq!(
                Ok(expected),
                decode_position(*position, num_interesting, data.len())
            );
        }
    }
}
