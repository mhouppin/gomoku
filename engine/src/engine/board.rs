use super::bitboard::Bitboard;
use super::types::Stone;

pub struct Board {
    bitsets: [Bitboard; 2],
    table: [Stone; 361],
    turn: Stone,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitsets: [Bitboard::new(); 2],
            table: [Stone::Empty; 361],
            turn: Stone::Black,
        }
    }
}
