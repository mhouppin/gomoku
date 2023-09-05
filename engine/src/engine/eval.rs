use super::score::Score;
use crate::core::{bitboard::BitboardIter, board::Board, magic::CrossAlignment, types::Square};

pub fn is_victory(board: &Board, sq: Square) -> bool {
    CrossAlignment::from(board, sq) == CrossAlignment::Five
}

pub fn score_alignment(alignment: CrossAlignment) -> Score {
    match alignment {
        CrossAlignment::Five => Score::cp(510),
        CrossAlignment::FourFour => Score::cp(380),
        CrossAlignment::OpenFourThree => Score::cp(300),
        CrossAlignment::FourThree => Score::cp(110),
        CrossAlignment::DoubleOpenThree => Score::cp(190),
        CrossAlignment::DoubleThree => Score::cp(40),
        CrossAlignment::OpenFour => Score::cp(240),
        CrossAlignment::Four => Score::cp(15),
        CrossAlignment::OpenThree => Score::cp(20),
        CrossAlignment::Three => Score::cp(5),
        CrossAlignment::NoAlign => Score::ZERO,
    }
}

pub fn evaluate(board: &Board) -> Score {
    let mut our_score = Score::ZERO;
    let mut their_score = Score::ZERO;

    for sq in BitboardIter::from_bitboard(&board.bitboard(board.turn())) {
        our_score += score_alignment(CrossAlignment::from(board, sq));
    }

    for sq in BitboardIter::from_bitboard(&board.bitboard(board.turn().flip())) {
        their_score += score_alignment(CrossAlignment::from(board, sq));
    }

    our_score - their_score + Score::cp(2)
}
