#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod engine;

use crate::core::board::Board;
use crate::core::types::{File, Rank, Square};

fn main() {
    let mut board = Board::new();

    board.push(Square::from(File::new(9), Rank::new(9)));
    board.push(Square::from(File::new(10), Rank::new(9)));
    board.push(Square::from(File::new(8), Rank::new(8)));
    board.push(Square::from(File::new(10), Rank::new(10)));
    board.push(Square::from(File::new(7), Rank::new(7)));
    board.push(Square::from(File::new(10), Rank::new(11)));
    board.push(Square::from(File::new(10), Rank::new(12)));

    println!("{}", board);
}
