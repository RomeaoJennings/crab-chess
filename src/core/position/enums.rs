#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    All,
}

impl PieceType {
    pub fn get_display_char(&self, player: Player) -> char {
        let white_pieces = ['♚', '♛', '♜', '♝', '♞', '♟'];
        let black_pieces = ['♔', '♕', '♖', '♗', '♘', '♙'];

        let index = *self as usize;
        match player {
            Player::White => white_pieces[index],
            Player::Black => black_pieces[index],
        }
    }

    pub fn iter() -> impl Iterator<Item = PieceType> {
        use PieceType::*;
        [King, Queen, Rook, Bishop, Knight, Pawn].into_iter()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Player {
    #[default]
    White,
    Black,
}

impl Player {
    pub fn other(&self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn iter() -> impl Iterator<Item = Player> {
        [Player::White, Player::Black].into_iter()
    }
}
