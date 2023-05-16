use super::bitboard::{Bitboard, BitboardIter};
use super::types::{Stone, BOARD_SIZE, Square};
use std::fmt;

#[derive(Debug)]
pub struct Board {
    bitsets: [Bitboard; 2],
    table: [Stone; BOARD_SIZE],
    turn: Stone,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "black:\n{}white:\n{}turn: {:?}\n",
            self.bitsets[0], self.bitsets[1], self.turn
        )
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitsets: [Bitboard::new(); 2],
            table: [Stone::Empty; BOARD_SIZE],
            turn: Stone::Black,
        }
    }

    pub fn from_raw_parts(black: Bitboard, white: Bitboard, turn: Stone) -> Self {
        let mut board = Self {
            bitsets: [black, white],
            table: [Stone::Empty; BOARD_SIZE],
            turn,
        };

        for square in BitboardIter::from_bitboard(&black) {
            board.table[square.value() as usize] = Stone::Black;
        }

        for square in BitboardIter::from_bitboard(&white) {
            board.table[square.value() as usize] = Stone::White;
        }

        board
    }

    pub fn bitboard(&self, side: Stone) -> Bitboard {
        match side {
            Stone::Black => self.bitsets[0],
            Stone::White => self.bitsets[1],
            Stone::Empty => !(self.bitsets[0] | self.bitsets[1]),
        }
    }

    pub fn push(&mut self, sq: Square) {
        self.table[sq.value() as usize] = self.turn;
        self.bitsets[if self.turn == Stone::Black { 0 } else { 1 }].set_square(sq);
        self.turn = self.turn.flip();
    }

    pub fn pop(&mut self, sq: Square) {
        self.turn = self.turn.flip();
        self.bitsets[if self.turn == Stone::Black { 0 } else { 1 }].rst_square(sq);
        self.table[sq.value() as usize] = Stone::Empty;
    }
}
