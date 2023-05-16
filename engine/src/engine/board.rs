use super::bitboard::Bitboard;
use super::types::{Stone, BOARD_SIZE};

pub struct Board {
    bitsets: [Bitboard; 2],
    table: [Stone; BOARD_SIZE],
    turn: Stone,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitsets: [Bitboard::new(); 2],
            table: [Stone::Empty; BOARD_SIZE],
            turn: Stone::Black,
        }
    }
}
