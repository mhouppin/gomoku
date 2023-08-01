use super::bitboard::{Bitboard, BitboardIter};
use super::types::{File, Rank, Square, Stone, BOARD_SIZE};
use std::fmt;

#[derive(Clone, Debug)]
pub struct Board {
    bitsets: [Bitboard; 2],
    table: [Stone; BOARD_SIZE],
    turn: Stone,
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
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

    pub fn setup_position(&mut self, grid: &str, turn: &str) {
        self.reset();

        let ranks = grid.split('/');

        for (rank_idx, rank_str) in ranks.enumerate() {
            let mut empty_span = 0;
            let rank = Rank::new(rank_idx as u8);
            let mut file = File::new(0);

            for b in rank_str.chars() {
                if let Some(value) = b.to_digit(10) {
                    empty_span = empty_span * 10 + value;
                } else {
                    file += empty_span as u8;
                    empty_span = 0;
                    if b == 'x' {
                        self.add_stone(Square::from(file, rank), Stone::Black);
                    } else if b == 'o' {
                        self.add_stone(Square::from(file, rank), Stone::White);
                    }
                    file += 1;
                }
            }
        }

        match turn {
            "w" => Stone::White,
            "b" => Stone::Black,
            _ => Stone::Black,
        };
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

    fn add_stone(&mut self, sq: Square, stone: Stone) {
        self.table[sq.value() as usize] = stone;
        self.bitsets[if stone == Stone::Black { 0 } else { 1 }].set_square(sq);
    }

    pub fn stone_at(&self, sq: Square) -> Stone {
        self.table[sq.value() as usize]
    }

    pub fn reset(&mut self) {
        self.bitsets = [Bitboard::new(); 2];
        self.table = [Stone::Empty; BOARD_SIZE];
        self.turn = Stone::Black;
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
