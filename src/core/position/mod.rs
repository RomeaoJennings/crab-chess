mod bitboards;
mod enums;

use std::fmt::Display;

use bitboards::PlayerBitboards;
pub use enums::{PieceType, Player};

#[derive(Debug, Clone)]
pub struct Position {
    bitboards: PlayerBitboards,
}

impl Default for Position {
    fn default() -> Self {
        Self { bitboards: Default::default() }
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut squares: [char; 64] = std::array::from_fn(|i| {
            let chars = ['■', '□'];
            let flip = (i / 8) % 2;
            let index = ((i % 2) + flip) % 2;
            chars[index]
        });
        for player in Player::iter() {
            for piece_type in PieceType::iter() {
                for index in self.bitboards[player][piece_type] {
                    squares[index] = piece_type.get_display_char(player);
                }
            }
        }
        writeln!(f, "╔═════════════════╗")?;
        for (i, c) in squares.into_iter().enumerate().rev() {
            if i % 8 == 7 {
                write!(f, "║ ")?;
            }
            write!(f, "{c} ")?;
            if i % 8 == 0 {
                writeln!(f, "║")?;
            }
        } 
        writeln!(f, "╚═════════════════╝")?;
        Ok(())
    }
}