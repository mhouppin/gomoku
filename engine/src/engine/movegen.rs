use rand::{rngs::ThreadRng, RngCore};

use crate::core::{
    bitboard::{Bitboard, BitboardIter},
    board::Board,
    types::{Square, Stone, BOARD_SIZE},
};

use std::fmt;

#[derive(Debug)]
pub struct Movegen {
    move_list: Vec<Square>,
}

impl Movegen {
    pub fn new() -> Self {
        Self {
            move_list: Vec::with_capacity(BOARD_SIZE),
        }
    }

    pub fn generate_near(&mut self, board: &Board) {
        let occupancy = board.bitboard(Stone::Black) | board.bitboard(Stone::White);
        let mut mask = occupancy;

        mask |= mask.shift_up() | mask.shift_down();
        mask |= mask.shift_left() | mask.shift_right();

        mask &= !occupancy;
        self.generate_from_mask(mask);
    }

    pub fn generate_far(&mut self, board: &Board) {
        let occupancy = board.bitboard(Stone::Black) | board.bitboard(Stone::White);
        let mut mask = occupancy;

        mask |= mask.shift_up() | mask.shift_down();
        mask |= mask.shift_left() | mask.shift_right();
        mask |= mask.shift_up() | mask.shift_down();
        mask |= mask.shift_left() | mask.shift_right();

        mask &= !occupancy;
        self.generate_from_mask(mask);
    }

    pub fn generate_all(&mut self, board: &Board) {
        self.generate_from_mask(board.bitboard(Stone::Empty));
    }

    pub fn generate_from_mask(&mut self, mask: Bitboard) {
        for square in BitboardIter::from_bitboard(&mask) {
            self.move_list.push(square)
        }
    }

    pub fn select_random_move(&self) -> Square {
        let mut rng = ThreadRng::default();

        self.move_list[rng.next_u64() as usize % self.move_list.len()]
    }
}

impl Default for Movegen {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Movegen {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.move_list.len() {
            write!(
                f,
                "{},{}",
                self.move_list[i],
                if i % 10 != 9 { ' ' } else { '\n' }
            )?;
        }
        Ok(())
    }
}
