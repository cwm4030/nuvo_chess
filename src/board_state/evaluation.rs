use crate::board_state::{board::Board, move_gen::generate_moves, piece_type::OFF_BOARD_SQUARE};

pub fn evaluate_board(board: &mut Board, stm_moves: usize) -> i16 {
    let mut score = 0_i16;

    score += (board.w_queens as i16 - board.b_queens as i16) * 900;
    score += (board.w_rooks as i16 - board.b_rooks as i16) * 500;
    score += (board.w_bishops as i16 - board.b_bishops as i16) * 300;
    score += (board.w_knights as i16 - board.b_knights as i16) * 300;
    score += (board.w_pawns as i16 - board.b_pawns as i16) * 100;

    board.stm ^= OFF_BOARD_SQUARE;
    let mi = generate_moves(board, false);
    board.stm ^= OFF_BOARD_SQUARE;
    score += (stm_moves as i16 - mi.c_move_list.count as i16) * 10;

    score
}
