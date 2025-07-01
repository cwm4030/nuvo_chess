use crate::board_state::{board::Board, square_index::RAW_INDEX_TO_64};

const W_PAWNS: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 50, 50, 50, 50, 50, 50, 50, 50, 10, 10, 20, 30, 30, 20, 10, 10, 5, 5,
    10, 25, 25, 10, 5, 5, 0, 0, 0, 20, 20, 0, 0, 0, 5, -5, -10, 0, 0, -10, -5, 5, 5, 10, 10, -20,
    -20, 10, 10, 5, 0, 0, 0, 0, 0, 0, 0, 0,
];
const B_PAWNS: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, -20, -20, 10, 10, 5, 5, -5, -10, 0, 0, -10, -5, 5, 0, 0, 0,
    20, 20, 0, 0, 0, 5, 5, 10, 25, 25, 10, 5, 5, 10, 10, 20, 30, 30, 20, 10, 10, 50, 50, 50, 50,
    50, 50, 50, 50, 0, 0, 0, 0, 0, 0, 0, 0,
];
const KNIGHTS: [i16; 64] = [
    -50, -40, -30, -30, -30, -30, -40, -50, -40, -20, 0, 0, 0, 0, -20, -40, -30, 0, 10, 15, 15, 10,
    0, -30, -30, 5, 15, 20, 20, 15, 5, -30, -30, 0, 15, 20, 20, 15, 0, -30, -30, 5, 10, 15, 15, 10,
    5, -30, -40, -20, 0, 5, 5, 0, -20, -40, -50, -40, -30, -30, -30, -30, -40, -50,
];
const BISHOPS: [i16; 64] = [
    -20, -10, -10, -10, -10, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 10, 10, 5, 0,
    -10, -10, 5, 5, 10, 10, 5, 5, -10, -10, 0, 10, 10, 10, 10, 0, -10, -10, 10, 10, 10, 10, 10, 10,
    -10, -10, 5, 0, 0, 0, 0, 5, -10, -20, -10, -10, -10, -10, -10, -10, -20,
];
const W_ROOKS: [i16; 64] = [
    0, 0, 0, 0, 0, 0, 0, 0, 5, 10, 10, 10, 10, 10, 10, 5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0,
    0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 0, 0,
    0, 5, 5, 0, 0, 0,
];
const B_ROOKS: [i16; 64] = [
    0, 0, 0, 5, 5, 0, 0, 0, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0,
    0, -5, -5, 0, 0, 0, 0, 0, 0, -5, -5, 0, 0, 0, 0, 0, 0, -5, 5, 10, 10, 10, 10, 10, 10, 5, 0, 0,
    0, 0, 0, 0, 0, 0,
];
const QUEENS: [i16; 64] = [
    -20, -10, -10, -5, -5, -10, -10, -20, -10, 0, 0, 0, 0, 0, 0, -10, -10, 0, 5, 5, 5, 5, 0, -10,
    -5, 0, 5, 5, 5, 5, 0, -5, 0, 0, 5, 5, 5, 5, 0, -5, -10, 5, 5, 5, 5, 5, 0, -10, -10, 0, 5, 0, 0,
    0, 0, -10, -20, -10, -10, -5, -5, -10, -10, -20,
];

pub fn evaluate_board(board: &Board) -> i16 {
    let mut score = 0_i16;

    score += (board.w_queens as i16 - board.b_queens as i16) * 900;
    score += (board.w_rooks as i16 - board.b_rooks as i16) * 500;
    score += (board.w_bishops as i16 - board.b_bishops as i16) * 330;
    score += (board.w_knights as i16 - board.b_knights as i16) * 320;
    score += (board.w_pawns as i16 - board.b_pawns as i16) * 100;

    score += pst_evaulate(&board.w_pawn_indexes, &W_PAWNS);
    score -= pst_evaulate(&board.b_pawn_indexes, &B_PAWNS);
    score += pst_evaulate(&board.w_knight_indexes, &KNIGHTS);
    score -= pst_evaulate(&board.b_knight_indexes, &KNIGHTS);
    score += pst_evaulate(&board.w_bishop_indexes, &BISHOPS);
    score -= pst_evaulate(&board.b_bishop_indexes, &BISHOPS);
    score += pst_evaulate(&board.w_rook_indexes, &W_ROOKS);
    score -= pst_evaulate(&board.b_rook_indexes, &B_ROOKS);
    score += pst_evaulate(&board.w_queen_indexes, &QUEENS);
    score -= pst_evaulate(&board.b_queen_indexes, &QUEENS);

    score
}

fn pst_evaulate(piece_indexes: &[u8], pst: &[i16; 64]) -> i16 {
    let mut score = 0_i16;
    for &index in piece_indexes {
        let pst_index = RAW_INDEX_TO_64[index as usize];
        score += pst[pst_index as usize];
    }
    score
}
