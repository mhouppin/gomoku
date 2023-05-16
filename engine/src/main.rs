#[macro_use]
extern crate lazy_static;

pub mod core;
pub mod engine;

use crate::core::bitboard::Bitboard;
use crate::core::board::Board;
use crate::core::types::{File, Rank, Square, Stone};
use crate::engine::movegen::Movegen;

fn main() {
    println!("{}", crate::core::magic::MAGIC_STRUCT.len());
}
