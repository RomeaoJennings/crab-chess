mod bitboard;
mod position;

pub use self::{bitboard::*, position::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Player {
    #[default]
    White,
    Black
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}