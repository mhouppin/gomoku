use crate::core::{board::Board, magic::CrossAlignment, types::Square};

pub fn is_victory(board: &Board, sq: Square) -> bool {
    CrossAlignment::from(board, sq) == CrossAlignment::Five
}
