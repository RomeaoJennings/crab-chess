use super::Bitboard;


pub struct IndexIterator(u64);

impl Iterator for IndexIterator {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        match self.0.trailing_zeros() as usize {
            64 => None,
            index => {
                self.0 &= self.0 - 1;
                Some(index)
            }
        }
    }
}

impl From<Bitboard> for IndexIterator {
    fn from(value: Bitboard) -> Self {
        Self(value.0)
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::IndexIterator;


    #[rstest]
    #[case::small_bits(0b01011, &[0, 1, 3])]
    #[case::empty(0, &[])]
    #[case::spread(0xF000000000020001, &[0, 17, 60, 61, 62, 63])]
    fn iteration(#[case] val: u64, #[case] expected_vals: &[usize]) {
        let mut iter = IndexIterator(val);

        for &val in expected_vals {
            assert_eq!(Some(val), iter.next());
        }
        assert_eq!(None, iter.next());
    }
}