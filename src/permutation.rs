use factorial::Factorial;
use std::cmp::Ord;
use std::convert::{TryFrom, TryInto};

struct PermutationCounts {
    // Each count represents the number of items positioned to the left of the value at this index
    // that are lower to that value.
    // If the original data is decreasingly sorted, these are all zero
    // Example:
    // sorted:   7, 6, 5, 4, 3, 2, 1, 0
    // shuffled: 3, 6, 5, 7, 0, 2, 1, 4
    // counts:   0, 1, 1, 3, 0, 1, 1, 4
    // permutation: 0*0! + 1*1! + 1*2! + 3*3! + 0*4! + 1*5! + 1*6! + 4*7! == 21021
    counts: Vec<usize>,
}

impl PermutationCounts {
    fn calculate_count<T: Ord>(pos: usize, x: &T, data: &[T]) -> usize {
        // Count items that are positioned to the left of this value, but are lower.
        // 0 in a descendingly sorted collection
        data.iter().take(pos).filter(|y| y < &x).count()
    }
    fn encode_count(indexed_count: (usize, usize)) -> usize {
        let (index, count) = indexed_count;
        count * index.factorial()
    }
    fn encode(&self) -> usize {
        self.counts
            .iter()
            .cloned()
            .enumerate()
            .map(Self::encode_count)
            .sum()
    }
    fn from_data<T: Ord>(data: &[T]) -> Self {
        Self {
            counts: data
                .iter()
                .enumerate()
                .map(|(index, x)| Self::calculate_count(index, x, data))
                .collect(),
        }
    }
    fn decode_count(index: usize, permutation: &mut usize) -> usize {
        let factorial = index.factorial();
        let count = permutation.div_euclid(factorial);
        *permutation = permutation.rem_euclid(factorial);
        count
    }
    fn decode(mut permutation: usize, n: usize) -> Self {
        // Must be decoded in reverse order as we'll remove parts from the permutation number
        let mut counts = (0..n)
            .rev()
            .map(|index| Self::decode_count(index, &mut permutation))
            .collect::<Vec<_>>();
        counts.reverse();
        Self { counts }
    }
    fn nth_smallest<T: Ord + Clone>(n: usize, increasing: &[T], permuted: &[T]) -> T {
        increasing
            .iter()
            .filter(|x| !permuted.contains(*x))
            .nth(n)
            .unwrap()
            .clone()
    }
    fn apply<T: Ord + Clone>(&self, data: &[T]) -> Vec<T> {
        let mut increasing = data.to_vec();
        increasing.sort();
        let mut permuted = Vec::new();
        for count in self.counts.iter().rev() {
            permuted.push(Self::nth_smallest(*count, &increasing, &permuted));
        }
        permuted.reverse();
        permuted
    }
}

/// Encode a slice's permutation from the ordered state as a single number.
///
/// From a slice of up to 20 elements, produce a unique number that represents the order of the
/// elements in that slice. This number can later be used to
///
/// This is the most space-efficient way of storing that information as every bit of entropy in the
/// slice's permutation is represented by a bit in the permutation number.
/// The maximum possible output is n!, where n is the size of the slice.
/// The usize can safely be downcast if n! can fit in a smaller integer.
/// 5! fits in a u8
/// 8! fits in a u16
/// 12! fits in a u32
/// 20! fits in a u64
pub fn encode_permutation<T: Ord, Encoded: TryFrom<usize>>(
    data: &[T],
) -> Result<Encoded, Encoded::Error> {
    PermutationCounts::from_data(data).encode().try_into()
}

/// Decode a permutation number into a unique slice's permutation
///
/// Take as input the permutation number produced by `encode_permutation` and a slice with each
/// element of the set in an arbitrary order.
///
/// Output a Vec with the data ordered in the unique permutation that matches this permutation
/// number
pub fn decode_permutation<'a, T: Ord + Clone, ToDecode: TryInto<usize>>(
    permutation: ToDecode,
    data: &'a [T],
) -> Result<Vec<T>, ToDecode::Error> {
    Ok(PermutationCounts::decode(permutation.try_into()?, data.len()).apply(&data))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    const SEQ: [u8; 8] = [3, 6, 5, 7, 0, 2, 1, 4];
    #[test]
    fn test_encode_permutation() {
        assert_eq!(Ok(21021), encode_permutation(&SEQ));
    }
    #[test]
    fn test_encode_permutation_fails_with_too_small_type() {
        assert!(encode_permutation::<_, u8>(&SEQ).is_err());
    }
    #[test]
    fn test_decode_permutation() {
        let seq = SEQ.to_vec();
        let mut rng = thread_rng();
        let mut shuffled = seq.clone();
        shuffled.shuffle(&mut rng);
        assert_eq!(decode_permutation(21021, &shuffled), Ok(seq));
    }
}
