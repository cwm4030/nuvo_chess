use crate::board_state::{
    board::Board,
    piece_type::{BLACK, WHITE},
    square_index::RAW_INDEX_TO_64,
};

const CENTER_PST: [i16; 64] = [
    0, 1, 2, 3, 3, 2, 1, 0, 1, 4, 5, 6, 6, 5, 4, 1, 2, 5, 7, 8, 8, 7, 5, 2, 3, 6, 8, 9, 9, 8, 6, 3,
    3, 6, 8, 9, 9, 8, 6, 3, 2, 5, 7, 8, 8, 7, 5, 2, 1, 4, 5, 6, 6, 5, 4, 1, 0, 1, 2, 3, 3, 2, 1, 0,
];

pub fn evaluate_board(board: &Board) -> i16 {
    let mut score = 0_i16;

    score += (board.w_queens as i16 - board.b_queens as i16) * 900;
    score += (board.w_rooks as i16 - board.b_rooks as i16) * 500;
    score += (board.w_bishops as i16 - board.b_bishops as i16) * 300;
    score += (board.w_knights as i16 - board.b_knights as i16) * 300;
    score += (board.w_pawns as i16 - board.b_pawns as i16) * 100;

    score +=
        (pst_evaulate(board, WHITE, &CENTER_PST) - pst_evaulate(board, BLACK, &CENTER_PST)) * 10;

    score
}

fn pst_evaulate(board: &Board, stm: u8, pst: &[i16; 64]) -> i16 {
    let mut score = 0_i16;
    if stm == WHITE {
        for i in 0..board.w_pawns {
            let index = RAW_INDEX_TO_64[board.w_pawn_indexes[i as usize] as usize];
            score += pst[index as usize];
        }

        for i in 0..board.w_knights {
            let index = RAW_INDEX_TO_64[board.w_knight_indexes[i as usize] as usize];
            score += pst[index as usize] * 3;
        }

        for i in 0..board.w_bishops {
            let index = RAW_INDEX_TO_64[board.w_bishop_indexes[i as usize] as usize];
            score += pst[index as usize] * 3;
        }

        for i in 0..board.w_rooks {
            let index = RAW_INDEX_TO_64[board.w_rook_indexes[i as usize] as usize];
            score += pst[index as usize] * 5;
        }

        for i in 0..board.w_queens {
            let index = RAW_INDEX_TO_64[board.w_queen_indexes[i as usize] as usize];
            score += pst[index as usize] * 9;
        }

        let index = RAW_INDEX_TO_64[board.w_king_index as usize];
        score += pst[index as usize] * 10;
    } else {
        for i in 0..board.b_pawns {
            let index = RAW_INDEX_TO_64[board.b_pawn_indexes[i as usize] as usize];
            score += pst[index as usize];
        }

        for i in 0..board.b_knights {
            let index = RAW_INDEX_TO_64[board.b_knight_indexes[i as usize] as usize];
            score += pst[index as usize] * 3;
        }

        for i in 0..board.b_bishops {
            let index = RAW_INDEX_TO_64[board.b_bishop_indexes[i as usize] as usize];
            score += pst[index as usize] * 3;
        }

        for i in 0..board.b_rooks {
            let index = RAW_INDEX_TO_64[board.b_rook_indexes[i as usize] as usize];
            score += pst[index as usize] * 5;
        }

        for i in 0..board.b_queens {
            let index = RAW_INDEX_TO_64[board.b_queen_indexes[i as usize] as usize];
            score += pst[index as usize] * 9;
        }

        let index = RAW_INDEX_TO_64[board.b_king_index as usize];
        score += pst[index as usize] * 10;
    }
    score
}
