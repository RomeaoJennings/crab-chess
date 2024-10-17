use std::ops::{Index, IndexMut};

use super::{Bitboard, Player};

#[derive(Debug, Clone)]
pub struct Position {
    player_to_move: Player,
    bitboards: PlayerBitboards,
}

#[derive(Debug, Clone)]
struct PlayerBitboards([PieceTypeBitboards; 2]);

impl PlayerBitboards {
    pub fn empty() -> Self {
        Self(std::array::from_fn(|_| PieceTypeBitboards::empty()))
    }
}

impl Default for PlayerBitboards {
    fn default() -> Self {
        let white_king = Bitboard::from(0x4);
        let white_queen = Bitboard::from(0x8);
        let white_rooks = Bitboard::from(0x81);
        let white_bishops = Bitboard::from(0x24);
        let white_knights = Bitboard::from(0x42);
        let white_pawns = Bitboard::from(0xFF00);
        let white_all = Bitboard::from(0xFFFF);

        let black_king = Bitboard::from(0x400000000000000);
        let black_queen = Bitboard::from(0x800000000000000);
        let black_rooks = Bitboard::from(0x8100000000000000);
        let black_bishops = Bitboard::from(0x2400000000000000);
        let black_knights = Bitboard::from(0x4200000000000000);
        let black_pawns = Bitboard::from(0xFF00000000000000);
        let black_all = Bitboard::from(0xFFFF000000000000);

        Self([
            PieceTypeBitboards([
                white_king,
                white_queen,
                white_rooks,
                white_bishops,
                white_knights,
                white_pawns,
                white_all,
            ]),
            PieceTypeBitboards([
                black_king,
                black_queen,
                black_rooks,
                black_bishops,
                black_knights,
                black_pawns,
                black_all,
            ]),
        ])
    }
}

impl Index<Player> for PlayerBitboards {
    type Output = PieceTypeBitboards;

    fn index(&self, index: Player) -> &Self::Output {
        let index = index as usize;
        &self.0[index]
    }
}

impl IndexMut<Player> for PlayerBitboards {
    fn index_mut(&mut self, index: Player) -> &mut Self::Output {
        let index = index as usize;
        &mut self.0[index]
    }
}

#[derive(Debug, Clone)]
struct PieceTypeBitboards([Bitboard; 7]);

impl PieceTypeBitboards {
    pub fn empty() -> Self {
        Self([Bitboard::from(0); 7])
    }
}

impl Index<PieceType> for PieceTypeBitboards {
    type Output = Bitboard;

    fn index(&self, index: PieceType) -> &Self::Output {
        let index = index as usize;
        &self.0[index]
    }
}

impl IndexMut<PieceType> for PieceTypeBitboards {
    fn index_mut(&mut self, index: PieceType) -> &mut Self::Output {
        let index = index as usize;
        &mut self.0[index]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    All,
}
