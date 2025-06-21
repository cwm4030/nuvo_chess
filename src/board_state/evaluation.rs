use crate::board_state::{
    board::Board, c_move_list::CMoveList, move_gen::generate_moves, piece_type::OFF_BOARD_SQUARE,
};

pub fn evaluate_board(board: &mut Board, stm_moves: usize) -> f32 {
    let mut score = 0.0;

    score += (board.w_queens - board.b_queens) as f32 * 9.0;
    score += (board.w_rooks - board.b_rooks) as f32 * 5.0;
    score += (board.w_bishops - board.b_bishops) as f32 * 3.0;
    score += (board.w_knights - board.b_knights) as f32 * 3.0;
    score += (board.w_pawns - board.b_pawns) as f32 * 1.0;

    let mut c_move_list = CMoveList::new();
    board.stm ^= OFF_BOARD_SQUARE;
    generate_moves(board, &mut c_move_list);
    board.stm ^= OFF_BOARD_SQUARE;
    score += (stm_moves as f32 - c_move_list.count as f32) * 0.1;

    score
}
