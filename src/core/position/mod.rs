mod bitboards;
mod enums;

use bitboards::PlayerBitboards;
pub use enums::{PieceType, Player};

#[derive(Debug, Clone)]
pub struct Position {
    player_to_move: Player,
    bitboards: PlayerBitboards,
}
