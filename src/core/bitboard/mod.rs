use std::fmt::Display;

use iterator::IndexIterator;

mod iterator;

const NUM_SQUARES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitboard(u64);

impl From<u64> for Bitboard {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl IntoIterator for Bitboard {
    type Item = usize;
    type IntoIter = IndexIterator;
    
    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl Display for Bitboard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let display_vals = ['-', 'X'];
        let mut mask = 1 << 63;
        for i in 0..NUM_SQUARES {
            let bit = self.0 & mask != 0;
            let index = if bit { 1 } else { 0 };
            write!(f, "{}", display_vals[index])?;
            if i % 8 == 7 {
                writeln!(f, "")?;
            }
            mask >>= 1;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    #[rstest]
    #[case(
        0,
        "--------\n--------\n--------\n--------\n--------\n--------\n--------\n--------\n"
    )]
    #[case(
        0xA000000070000002,
        "X-X-----\n--------\n--------\n--------\n-XXX----\n--------\nt --------\n------X-\n"
    )]

    fn display_works(#[case] value: u64, #[case] expected: &str) {
        use super::Bitboard;

        let bitboard = Bitboard::from(value);
        let actual = bitboard.to_string();
        assert_eq!(actual, expected);
    }
}
