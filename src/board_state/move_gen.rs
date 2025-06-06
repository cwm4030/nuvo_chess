use crate::board_state::{
    board::Board,
    c_move_list::CMoveList,
    piece_type::{EMPTY_SQUARE, OFF_BOARD_SQUARE, WHITE, KNIGHT, QUEEN},
    square_index::{A8, H1},
};

const RAY_DETECTION_OFFSET: i8 = (-1 * (A8 as i16 - H1 as i16)) as i8;
const RAY_DETECTION: [i8; 240] = [
    17, 0, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 0, 15, 0, 0, 17, 0, 0, 0, 0, 0, 16, 0, 0, 0, 0, 0, 15,
    0, 0, 0, 0, 17, 0, 0, 0, 0, 16, 0, 0, 0, 0, 15, 0, 0, 0, 0, 0, 0, 17, 0, 0, 0, 16, 0, 0, 0, 15,
    0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 0, 16, 0, 0, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 0, 16, 0, 15,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 16, 15, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 1, 1, 1, 0,
    -1, -1, -1, -1, -1, -1, -1, 0, 0, 0, 0, 0, 0, 0, -15, -16, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, -15, 0, -16, 0, -17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -15, 0, 0, -16, 0, 0, -17, 0, 0, 0, 0,
    0, 0, 0, 0, -15, 0, 0, 0, -16, 0, 0, 0, -17, 0, 0, 0, 0, 0, 0, -15, 0, 0, 0, 0, -16, 0, 0, 0,
    0, -17, 0, 0, 0, 0, -15, 0, 0, 0, 0, 0, -16, 0, 0, 0, 0, 0, -17, 0, 0, -15, 0, 0, 0, 0, 0, 0,
    -16, 0, 0, 0, 0, 0, 0, -17, 0,
];
const PAWN_START_SQUARES: [u8; 192] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 16, 16, 16, 16, 16, 16, 16, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0,
];
const PAWN_PROMOTION_SQUARES: [u8; 192] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 16, 16, 16, 16, 16,
    16, 16, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0,
];
const PAWN_WHITE_DIRECTIONS: [i16; 4] = [-16, -32, -17, -15];
const PAWN_BLACK_DIRECTIONS: [i16; 4] = [16, 32, 17, 15];
const KNIGHT_DIRECTIONS: [i16; 8] = [-18, -33, -31, -14, 14, 31, 33, 18];
const BISHOP_DIRECTIONS: [i16; 4] = [-17, -15, 15, 17];
const ROOK_DIRECTIONS: [i16; 4] = [-16, -1, 1, 16];
const QUEEN_DIRECTIONS: [i16; 8] = [-17, -16, -15, -1, 1, 15, 16, 17];
const KING_DIRECTIONS: [i16; 8] = [-17, -16, -15, -1, 1, 15, 16, 17];

pub fn generate_moves(board: &Board, c_move_list: &mut CMoveList) {
    c_move_list.clear();

    if board.stm == WHITE {
        for i in 0..board.w_pawns {
            generate_non_capture_pawn_moves(board, board.w_pawn_indexes[i as usize], &PAWN_WHITE_DIRECTIONS, c_move_list);
            generate_capture_pawn_moves(board, board.w_pawn_indexes[i as usize], &PAWN_WHITE_DIRECTIONS, c_move_list);
        }

        for i in 0..board.w_knights {
            generate_non_sliding_moves(
                board,
                board.w_knight_indexes[i as usize],
                &KNIGHT_DIRECTIONS,
                c_move_list,
            );
        }

        for i in 0..board.w_bishops {
            generate_sliding_moves(
                board,
                board.w_bishop_indexes[i as usize],
                &BISHOP_DIRECTIONS,
                c_move_list,
            );
        }

        for i in 0..board.w_rooks {
            generate_sliding_moves(
                board,
                board.w_rook_indexes[i as usize],
                &ROOK_DIRECTIONS,
                c_move_list,
            );
        }

        for i in 0..board.w_queens {
            generate_sliding_moves(
                board,
                board.w_queen_indexes[i as usize],
                &QUEEN_DIRECTIONS,
                c_move_list,
            );
        }

        generate_non_sliding_moves(board, board.w_king_index, &KING_DIRECTIONS, c_move_list);
    } else {
        for i in 0..board.b_pawns {
            generate_non_capture_pawn_moves(board, board.b_pawn_indexes[i as usize], &PAWN_BLACK_DIRECTIONS, c_move_list);
            generate_capture_pawn_moves(board, board.b_pawn_indexes[i as usize], &PAWN_BLACK_DIRECTIONS, c_move_list);
        }

        for i in 0..board.b_knights {
            generate_non_sliding_moves(
                board,
                board.b_knight_indexes[i as usize],
                &KNIGHT_DIRECTIONS,
                c_move_list,
            );
        }

        for i in 0..board.b_bishops {
            generate_sliding_moves(
                board,
                board.b_bishop_indexes[i as usize],
                &BISHOP_DIRECTIONS,
                c_move_list,
            );
        }

        for i in 0..board.b_rooks {
            generate_sliding_moves(
                board,
                board.b_rook_indexes[i as usize],
                &ROOK_DIRECTIONS,
                c_move_list,
            );
        }

        for i in 0..board.b_queens {
            generate_sliding_moves(
                board,
                board.b_queen_indexes[i as usize],
                &QUEEN_DIRECTIONS,
                c_move_list,
            );
        }

        generate_non_sliding_moves(board, board.b_king_index, &KING_DIRECTIONS, c_move_list);
    }
}

fn generate_non_capture_pawn_moves(board: &Board, from_index: u8, directions: &[i16], c_move_list: &mut CMoveList) {
    let up_one_index = (from_index as i16 + directions[0]) as u8;
    let up_one_square = board.squares[up_one_index as usize];
    if up_one_square == EMPTY_SQUARE {
        if PAWN_PROMOTION_SQUARES[up_one_index as usize] == board.stm {
            for promotion_piece in KNIGHT..=QUEEN {
                c_move_list.add_move(from_index, up_one_index, promotion_piece);
            }
        } else {
            c_move_list.add_move(from_index, up_one_index, 0);
        }

        let up_two_index = (from_index as i16 + directions[1]) as u8;
        let up_two_square = board.squares[up_two_index as usize];
        if up_two_square == EMPTY_SQUARE && PAWN_START_SQUARES[from_index as usize] == board.stm {
            c_move_list.add_move(from_index, up_two_index, 0);
        }
    }
}

fn generate_capture_pawn_moves(board: &Board, from_index: u8, directions: &[i16], c_move_list: &mut CMoveList) {
    for direction in &directions[2..] {
        let to_index = (from_index as i16 + direction) as u8;
        let to_square = board.squares[to_index as usize];
        if to_square != EMPTY_SQUARE && to_square & OFF_BOARD_SQUARE != board.stm {
            if PAWN_PROMOTION_SQUARES[to_index as usize] == board.stm {
                for promotion_piece in KNIGHT..=QUEEN {
                    c_move_list.add_move(from_index, to_index, promotion_piece);
                }
            } else {
                c_move_list.add_move(from_index, to_index, 0);
            }
        }
    }
}

fn generate_non_sliding_moves(
    board: &Board,
    from_index: u8,
    directions: &[i16],
    c_move_list: &mut CMoveList,
) {
    for &direction in directions {
        let to_index = (from_index as i16 + direction) as u8;
        let to_square = board.squares[to_index as usize];
        if to_square == EMPTY_SQUARE || to_square & OFF_BOARD_SQUARE != board.stm {
            c_move_list.add_move(from_index, to_index, 0);
        }
    }
}

fn generate_sliding_moves(
    board: &Board,
    from_index: u8,
    directions: &[i16],
    c_move_list: &mut CMoveList,
) {
    for &direction in directions {
        let mut to_index = from_index as i16 + direction;
        let mut to_square = board.squares[to_index as usize];
        while to_square == EMPTY_SQUARE {
            c_move_list.add_move(from_index, to_index as u8, 0);
            to_index += direction;
            to_square = board.squares[to_index as usize];
        }

        if to_square & OFF_BOARD_SQUARE != board.stm {
            c_move_list.add_move(from_index, to_index as u8, 0);
        }
    }
}
