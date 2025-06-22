use crate::board_state::{
    board::Board,
    c_move_list::CMoveList,
    castling::{BLACK_KING, BLACK_QUEEN, WHITE_KING, WHITE_QUEEN},
    piece_type::{
        BISHOP, BLACK, EMPTY_SQUARE, KING, KNIGHT, OFF_BOARD_SQUARE, PAWN, PIECE_MASK, QUEEN, ROOK,
        WHITE,
    },
    square_index::{A8, B1, B8, C1, C8, D1, D8, F1, F8, G1, G8, H1},
};

const PINNER: u8 = 15;
const PIN: u8 = 16;
const EP_PIN: u8 = 32;
const DEFEND: u8 = 64;
const LOOKUP_OFFSET: i16 = -(A8 as i16 - H1 as i16);
const ATTACK_DIRECTION_LOOKUP: [i16; 240] = [
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
const SLIDER_ATTACK_LOOKUP: [u8; 240] = [
    3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 3, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 0,
    0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 0, 0, 4, 4, 4, 4, 4, 4, 4, 0, 4, 4, 4, 4, 4, 4, 4, 0,
    0, 0, 0, 0, 0, 0, 3, 4, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 4, 0, 3, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 3, 0, 0, 4, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 4, 0, 0, 0, 3, 0, 0, 0, 0,
    0, 0, 3, 0, 0, 0, 0, 4, 0, 0, 0, 0, 3, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 3, 0, 0,
    3, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 3, 0,
];
const NON_SLIDER_ATTACK_LOOKUP: [u8; 240] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 6, 6, 6, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 6, 0, 6, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 2, 6, 6, 6, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];
const PAWN_START_SQUARES: [u8; 192] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 16, 16, 16, 16, 16, 16, 16, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];
const PAWN_PROMOTION_SQUARES: [u8; 192] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 8, 8, 8, 8, 8, 8, 8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 16, 16, 16, 16, 16, 16, 16, 16, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0,
];
const PAWN_WHITE_DIRECTIONS: [i16; 4] = [-16, -32, -17, -15];
const PAWN_BLACK_DIRECTIONS: [i16; 4] = [16, 32, 17, 15];
const KNIGHT_DIRECTIONS: [i16; 8] = [-18, -33, -31, -14, 14, 31, 33, 18];
const BISHOP_DIRECTIONS: [i16; 4] = [-17, -15, 15, 17];
const ROOK_DIRECTIONS: [i16; 4] = [-16, -1, 1, 16];
const QUEEN_DIRECTIONS: [i16; 8] = [-17, -16, -15, -1, 1, 15, 16, 17];
const KING_DIRECTIONS: [i16; 8] = [-17, -16, -15, -1, 1, 15, 16, 17];

const ATTACKER_VICTIM_SCORES: [[u16; 7]; 7] = [
    [0, 0, 0, 0, 0, 0, 0],
    [0, 105, 205, 305, 405, 505, 605],
    [0, 104, 204, 304, 404, 504, 604],
    [0, 103, 203, 303, 403, 503, 603],
    [0, 102, 202, 302, 402, 502, 602],
    [0, 101, 201, 301, 401, 501, 601],
    [0, 100, 200, 300, 400, 500, 600],
];

pub struct MoveInformation {
    pub c_move_list: CMoveList,
    pub move_scores: [u16; 256],
    pub pin_defend_map: [u8; 192],
    pub check_count: u8,
}

impl MoveInformation {
    pub fn new() -> Self {
        MoveInformation {
            c_move_list: CMoveList::new(),
            move_scores: [0; 256],
            pin_defend_map: [0; 192],
            check_count: 0,
        }
    }

    pub fn sort_by_score(&mut self) {
        self.c_move_list.sort_by_score(&self.move_scores);
    }
}

pub fn generate_moves(board: &Board, apply_score: bool) -> MoveInformation {
    let mut mi = MoveInformation::new();
    set_pin_defend_map(&mut mi, board);

    if board.stm == WHITE {
        if mi.check_count <= 1 {
            for i in 0..board.w_pawns {
                generate_non_capture_pawn_moves(
                    &mut mi,
                    board,
                    board.w_pawn_indexes[i as usize],
                    &PAWN_WHITE_DIRECTIONS,
                    apply_score,
                );
                generate_capture_pawn_moves(
                    &mut mi,
                    board,
                    board.w_pawn_indexes[i as usize],
                    &PAWN_WHITE_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.w_knights {
                generate_non_sliding_moves(
                    &mut mi,
                    board,
                    board.w_knight_indexes[i as usize],
                    &KNIGHT_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.w_bishops {
                generate_sliding_moves(
                    &mut mi,
                    board,
                    board.w_bishop_indexes[i as usize],
                    &BISHOP_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.w_rooks {
                generate_sliding_moves(
                    &mut mi,
                    board,
                    board.w_rook_indexes[i as usize],
                    &ROOK_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.w_queens {
                generate_sliding_moves(
                    &mut mi,
                    board,
                    board.w_queen_indexes[i as usize],
                    &QUEEN_DIRECTIONS,
                    apply_score,
                );
            }
        }

        generate_non_sliding_moves(
            &mut mi,
            board,
            board.w_king_index,
            &KING_DIRECTIONS,
            apply_score,
        );
    } else {
        if mi.check_count <= 1 {
            for i in 0..board.b_pawns {
                generate_non_capture_pawn_moves(
                    &mut mi,
                    board,
                    board.b_pawn_indexes[i as usize],
                    &PAWN_BLACK_DIRECTIONS,
                    apply_score,
                );
                generate_capture_pawn_moves(
                    &mut mi,
                    board,
                    board.b_pawn_indexes[i as usize],
                    &PAWN_BLACK_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.b_knights {
                generate_non_sliding_moves(
                    &mut mi,
                    board,
                    board.b_knight_indexes[i as usize],
                    &KNIGHT_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.b_bishops {
                generate_sliding_moves(
                    &mut mi,
                    board,
                    board.b_bishop_indexes[i as usize],
                    &BISHOP_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.b_rooks {
                generate_sliding_moves(
                    &mut mi,
                    board,
                    board.b_rook_indexes[i as usize],
                    &ROOK_DIRECTIONS,
                    apply_score,
                );
            }

            for i in 0..board.b_queens {
                generate_sliding_moves(
                    &mut mi,
                    board,
                    board.b_queen_indexes[i as usize],
                    &QUEEN_DIRECTIONS,
                    apply_score,
                );
            }
        }

        generate_non_sliding_moves(
            &mut mi,
            board,
            board.b_king_index,
            &KING_DIRECTIONS,
            apply_score,
        );
    }
    generate_castle_moves(board, &mut mi);
    mi
}

fn generate_non_capture_pawn_moves(
    mi: &mut MoveInformation,
    board: &Board,
    from_index: u8,
    directions: &[i16],
    apply_score: bool,
) {
    let up_one_index = (from_index as i16 + directions[0]) as u8;
    let up_one_square = board.squares[up_one_index as usize];
    if up_one_square == EMPTY_SQUARE {
        if PAWN_PROMOTION_SQUARES[up_one_index as usize] == board.stm {
            for promotion_piece in KNIGHT..=QUEEN {
                add_legal_move(
                    mi,
                    board,
                    from_index,
                    up_one_index,
                    promotion_piece,
                    apply_score,
                );
            }
        } else {
            add_legal_move(mi, board, from_index, up_one_index, 0, apply_score);
        }

        let up_two_index = (from_index as i16 + directions[1]) as u8;
        let up_two_square = board.squares[up_two_index as usize];
        if up_two_square == EMPTY_SQUARE && PAWN_START_SQUARES[from_index as usize] == board.stm {
            add_legal_move(mi, board, from_index, up_two_index, 0, apply_score);
        }
    }
}

fn generate_capture_pawn_moves(
    mi: &mut MoveInformation,
    board: &Board,
    from_index: u8,
    directions: &[i16],
    apply_score: bool,
) {
    for direction in &directions[2..] {
        let to_index = (from_index as i16 + direction) as u8;
        let to_square = board.squares[to_index as usize];
        if to_square != EMPTY_SQUARE
            && to_square & OFF_BOARD_SQUARE == (board.stm ^ OFF_BOARD_SQUARE)
        {
            if PAWN_PROMOTION_SQUARES[to_index as usize] == board.stm {
                for promotion_piece in KNIGHT..=QUEEN {
                    add_legal_move(
                        mi,
                        board,
                        from_index,
                        to_index,
                        promotion_piece,
                        apply_score,
                    );
                }
            } else {
                add_legal_move(mi, board, from_index, to_index, 0, apply_score);
            }
        } else if to_index == board.ep_index {
            add_legal_move(mi, board, from_index, to_index, 0, apply_score);
        }
    }
}

fn generate_non_sliding_moves(
    mi: &mut MoveInformation,
    board: &Board,
    from_index: u8,
    directions: &[i16],
    apply_score: bool,
) {
    for &direction in directions {
        let to_index = (from_index as i16 + direction) as u8;
        let to_square = board.squares[to_index as usize];
        if to_square == EMPTY_SQUARE
            || to_square & OFF_BOARD_SQUARE == (board.stm ^ OFF_BOARD_SQUARE)
        {
            add_legal_move(mi, board, from_index, to_index, 0, apply_score);
        }
    }
}

fn generate_sliding_moves(
    mi: &mut MoveInformation,
    board: &Board,
    from_index: u8,
    directions: &[i16],
    apply_score: bool,
) {
    for &direction in directions {
        let mut to_index = from_index as i16 + direction;
        let mut to_square = board.squares[to_index as usize];
        while to_square == EMPTY_SQUARE {
            add_legal_move(mi, board, from_index, to_index as u8, 0, apply_score);
            to_index += direction;
            to_square = board.squares[to_index as usize];
        }

        if to_square & OFF_BOARD_SQUARE == (board.stm ^ OFF_BOARD_SQUARE) {
            add_legal_move(mi, board, from_index, to_index as u8, 0, apply_score);
        }
    }
}

fn generate_castle_moves(board: &Board, mi: &mut MoveInformation) {
    if mi.check_count > 0 {
        return;
    }

    if board.stm == WHITE {
        if board.castling_rights & WHITE_KING != 0
            && board.squares[F1 as usize] == EMPTY_SQUARE
            && board.squares[G1 as usize] == EMPTY_SQUARE
            && !is_square_attacked(board, F1)
            && !is_square_attacked(board, G1)
        {
            mi.c_move_list.add_move(board.w_king_index, G1, 0);
        }

        if board.castling_rights & WHITE_QUEEN != 0
            && board.squares[D1 as usize] == EMPTY_SQUARE
            && board.squares[C1 as usize] == EMPTY_SQUARE
            && board.squares[B1 as usize] == EMPTY_SQUARE
            && !is_square_attacked(board, D1)
            && !is_square_attacked(board, C1)
        {
            mi.c_move_list.add_move(board.w_king_index, C1, 0);
        }
    } else {
        if board.castling_rights & BLACK_KING != 0
            && board.squares[F8 as usize] == EMPTY_SQUARE
            && board.squares[G8 as usize] == EMPTY_SQUARE
            && !is_square_attacked(board, F8)
            && !is_square_attacked(board, G8)
        {
            mi.c_move_list.add_move(board.b_king_index, G8, 0);
        }

        if board.castling_rights & BLACK_QUEEN != 0
            && board.squares[D8 as usize] == EMPTY_SQUARE
            && board.squares[C8 as usize] == EMPTY_SQUARE
            && board.squares[B8 as usize] == EMPTY_SQUARE
            && !is_square_attacked(board, D8)
            && !is_square_attacked(board, C8)
        {
            mi.c_move_list.add_move(board.b_king_index, C8, 0);
        }
    }
}

fn add_legal_move(
    mi: &mut MoveInformation,
    board: &Board,
    from_index: u8,
    to_index: u8,
    promotion_piece: u8,
    apply_score: bool,
) {
    let from_square = board.squares[from_index as usize];
    let to_square = board.squares[to_index as usize];
    if from_square & PIECE_MASK == KING {
        if is_square_attacked(board, to_index) {
            return;
        }
    } else if mi.check_count > 1 {
        return;
    } else if mi.check_count == 1 {
        if (to_index == board.ep_index
            && mi.pin_defend_map[board.ep_index as usize] & DEFEND == DEFEND
            && from_square & PIECE_MASK != PAWN)
            || mi.pin_defend_map[to_index as usize] & DEFEND != DEFEND
            || mi.pin_defend_map[from_index as usize] & PIN == PIN
        {
            return;
        }
    } else if (mi.pin_defend_map[from_index as usize] & PIN == PIN
        && (mi.pin_defend_map[from_index as usize] & PINNER
            != mi.pin_defend_map[to_index as usize] & PINNER))
        || (from_square & PIECE_MASK == PAWN
            && to_index == board.ep_index
            && mi.pin_defend_map[to_index as usize] & EP_PIN == EP_PIN)
    {
        return;
    }

    mi.c_move_list
        .add_move(from_index, to_index, promotion_piece);
    if apply_score {
        let attack_piece = from_square & PIECE_MASK;
        let victim_piece = to_square & PIECE_MASK;
        let score = ATTACKER_VICTIM_SCORES[attack_piece as usize][victim_piece as usize];
        mi.move_scores[mi.c_move_list.count - 1] = score
    }
}

fn set_pin_defend_map(mi: &mut MoveInformation, board: &Board) {
    let mut pinner: u8 = 1;

    if board.stm == WHITE {
        for &direction in &PAWN_WHITE_DIRECTIONS[2..] {
            if mi.check_count > 1 {
                return;
            }
            let attack_index = (board.w_king_index as i16 + direction) as u8;
            if board.squares[attack_index as usize] == BLACK | PAWN {
                mi.check_count += 1;
                mi.pin_defend_map[attack_index as usize] |= DEFEND;
            }
        }
        let ep_pawn_index = get_ep_pawn_index(board);
        if mi.pin_defend_map[ep_pawn_index as usize] & DEFEND == DEFEND {
            mi.pin_defend_map[board.ep_index as usize] |= DEFEND;
        }

        for i in 0..board.b_knights {
            if mi.check_count > 1 {
                return;
            }
            let from_index = board.b_knight_indexes[i as usize];
            let attack_index =
                (from_index as i16 - board.w_king_index as i16 + LOOKUP_OFFSET) as usize;
            if NON_SLIDER_ATTACK_LOOKUP[attack_index] == KNIGHT {
                mi.check_count += 1;
                mi.pin_defend_map[from_index as usize] |= DEFEND;
            }
        }

        set_slider_pin_defend_map(
            mi,
            board,
            &board.b_bishop_indexes[..board.b_bishops as usize],
            board.w_king_index,
            BISHOP,
            &mut pinner,
        );
        if mi.check_count > 1 {
            return;
        }

        set_slider_pin_defend_map(
            mi,
            board,
            &board.b_rook_indexes[..board.b_rooks as usize],
            board.w_king_index,
            ROOK,
            &mut pinner,
        );
        if mi.check_count > 1 {
            return;
        }

        set_slider_pin_defend_map(
            mi,
            board,
            &board.b_queen_indexes[..board.b_queens as usize],
            board.w_king_index,
            QUEEN,
            &mut pinner,
        );
    } else {
        for &direction in &PAWN_BLACK_DIRECTIONS[2..] {
            if mi.check_count > 1 {
                return;
            }
            let attack_index = (board.b_king_index as i16 + direction) as u8;
            if board.squares[attack_index as usize] == WHITE | PAWN {
                mi.check_count += 1;
                mi.pin_defend_map[attack_index as usize] |= DEFEND;
            }
        }
        let ep_pawn_index = get_ep_pawn_index(board);
        if mi.pin_defend_map[ep_pawn_index as usize] & DEFEND == DEFEND {
            mi.pin_defend_map[board.ep_index as usize] |= DEFEND;
        }

        for i in 0..board.w_knights {
            if mi.check_count > 1 {
                return;
            }
            let from_index = board.w_knight_indexes[i as usize];
            let attack_index =
                (from_index as i16 - board.b_king_index as i16 + LOOKUP_OFFSET) as usize;
            if NON_SLIDER_ATTACK_LOOKUP[attack_index] == KNIGHT {
                mi.check_count += 1;
                mi.pin_defend_map[from_index as usize] |= DEFEND;
            }
        }

        set_slider_pin_defend_map(
            mi,
            board,
            &board.w_bishop_indexes[..board.w_bishops as usize],
            board.b_king_index,
            BISHOP,
            &mut pinner,
        );
        if mi.check_count > 1 {
            return;
        }

        set_slider_pin_defend_map(
            mi,
            board,
            &board.w_rook_indexes[..board.w_rooks as usize],
            board.b_king_index,
            ROOK,
            &mut pinner,
        );
        if mi.check_count > 1 {
            return;
        }

        set_slider_pin_defend_map(
            mi,
            board,
            &board.w_queen_indexes[..board.w_queens as usize],
            board.b_king_index,
            QUEEN,
            &mut pinner,
        );
    }
}

fn set_slider_pin_defend_map(
    mi: &mut MoveInformation,
    board: &Board,
    slider_indexes: &[u8],
    stm_king_index: u8,
    slider_type: u8,
    pinner: &mut u8,
) {
    for &from_index in slider_indexes {
        if mi.check_count > 1 {
            return;
        }
        let attack_index = (from_index as i16 - stm_king_index as i16 + LOOKUP_OFFSET) as usize;
        let attack_piece = SLIDER_ATTACK_LOOKUP[attack_index];
        if attack_piece == 0 || (slider_type != QUEEN && attack_piece != slider_type) {
            continue;
        }

        let attack_direction = ATTACK_DIRECTION_LOOKUP[attack_index];
        let mut to_index = from_index as i16 + attack_direction;
        while board.squares[to_index as usize] == EMPTY_SQUARE {
            to_index += attack_direction;
        }

        let to_square = board.squares[to_index as usize];
        let next_to_index = to_index + attack_direction;
        let next_to_square = board.squares[next_to_index as usize];
        if to_index as u8 == stm_king_index {
            mi.check_count += 1;
            if mi.check_count > 1 {
                return;
            }

            let mut defend_index = from_index as i16;
            while defend_index != to_index {
                mi.pin_defend_map[defend_index as usize] |= DEFEND;
                defend_index += attack_direction;
            }
        } else if board.ep_index != 0
            && slider_type == ROOK
            && (attack_direction == -1 || attack_direction == 1)
            && to_square & PIECE_MASK == PAWN
            && next_to_square & PIECE_MASK == PAWN
            && (to_square | next_to_square) & OFF_BOARD_SQUARE == OFF_BOARD_SQUARE
        {
            let ep_pawn_index = get_ep_pawn_index(board);
            if to_index as u8 != ep_pawn_index && next_to_index as u8 != ep_pawn_index {
                continue;
            }

            to_index += attack_direction;
            to_index += attack_direction;
            while board.squares[to_index as usize] == EMPTY_SQUARE {
                to_index += attack_direction;
            }

            if to_index as u8 == stm_king_index {
                mi.pin_defend_map[board.ep_index as usize] |= EP_PIN;
            }
        } else if to_square & OFF_BOARD_SQUARE == board.stm {
            let pin_index = to_index;
            to_index += attack_direction;
            while board.squares[to_index as usize] == EMPTY_SQUARE {
                to_index += attack_direction;
            }

            if to_index as u8 == stm_king_index {
                mi.pin_defend_map[pin_index as usize] |= *pinner | PIN;
                if board.squares[pin_index as usize] & PIECE_MASK == KNIGHT {
                    *pinner += 1;
                    continue;
                }

                let mut pinner_index = from_index as i16;
                while pinner_index as u8 != stm_king_index {
                    mi.pin_defend_map[pinner_index as usize] |= *pinner;
                    pinner_index += attack_direction;
                }
                *pinner += 1;
            }
        }
    }
}

fn is_square_attacked(board: &Board, square_index: u8) -> bool {
    if board.stm == WHITE {
        for &direction in &PAWN_WHITE_DIRECTIONS[2..] {
            let attack_index = (square_index as i16 + direction) as u8;
            if board.squares[attack_index as usize] == BLACK | PAWN {
                return true;
            }
        }

        for i in 0..board.b_knights {
            let from_index = board.b_knight_indexes[i as usize];
            let attack_index = (from_index as i16 - square_index as i16 + LOOKUP_OFFSET) as usize;
            if NON_SLIDER_ATTACK_LOOKUP[attack_index] == KNIGHT {
                return true;
            }
        }

        if is_slider_attacking(
            board,
            &board.b_bishop_indexes[..board.b_bishops as usize],
            square_index,
            BISHOP,
            board.w_king_index,
        ) {
            return true;
        }

        if is_slider_attacking(
            board,
            &board.b_rook_indexes[..board.b_rooks as usize],
            square_index,
            ROOK,
            board.w_king_index,
        ) {
            return true;
        }

        if is_slider_attacking(
            board,
            &board.b_queen_indexes[..board.b_queens as usize],
            square_index,
            QUEEN,
            board.w_king_index,
        ) {
            return true;
        }

        let from_index = board.b_king_index;
        let attack_index = (from_index as i16 - square_index as i16 + LOOKUP_OFFSET) as usize;
        if NON_SLIDER_ATTACK_LOOKUP[attack_index] == KING {
            return true;
        }
    } else {
        for &direction in &PAWN_BLACK_DIRECTIONS[2..] {
            let attack_index = (square_index as i16 + direction) as u8;
            if board.squares[attack_index as usize] == WHITE | PAWN {
                return true;
            }
        }

        for i in 0..board.w_knights {
            let from_index = board.w_knight_indexes[i as usize];
            let attack_index = (from_index as i16 - square_index as i16 + LOOKUP_OFFSET) as usize;
            if NON_SLIDER_ATTACK_LOOKUP[attack_index] == KNIGHT {
                return true;
            }
        }

        if is_slider_attacking(
            board,
            &board.w_bishop_indexes[..board.w_bishops as usize],
            square_index,
            BISHOP,
            board.b_king_index,
        ) {
            return true;
        }

        if is_slider_attacking(
            board,
            &board.w_rook_indexes[..board.w_rooks as usize],
            square_index,
            ROOK,
            board.b_king_index,
        ) {
            return true;
        }

        if is_slider_attacking(
            board,
            &board.w_queen_indexes[..board.w_queens as usize],
            square_index,
            QUEEN,
            board.b_king_index,
        ) {
            return true;
        }

        let from_index = board.w_king_index;
        let attack_index = (from_index as i16 - square_index as i16 + LOOKUP_OFFSET) as usize;
        if NON_SLIDER_ATTACK_LOOKUP[attack_index] == KING {
            return true;
        }
    }
    false
}

fn is_slider_attacking(
    board: &Board,
    slider_indexes: &[u8],
    square_index: u8,
    slider_type: u8,
    stm_king_index: u8,
) -> bool {
    for &from_index in slider_indexes {
        let attack_index = (from_index as i16 - square_index as i16 + LOOKUP_OFFSET) as usize;
        let attack_piece = SLIDER_ATTACK_LOOKUP[attack_index];
        if attack_piece == 0 || (slider_type != QUEEN && attack_piece != slider_type) {
            continue;
        }

        let attack_direction = ATTACK_DIRECTION_LOOKUP[attack_index];
        let mut to_index = from_index as i16 + attack_direction;
        while to_index as u8 != square_index && board.squares[to_index as usize] == EMPTY_SQUARE
            || to_index as u8 == stm_king_index
        {
            to_index += attack_direction;
        }

        if to_index as u8 == square_index {
            return true;
        }
    }
    false
}

fn get_ep_pawn_index(board: &Board) -> u8 {
    if board.ep_index == 0 {
        return 0;
    }

    match board.stm {
        WHITE => board.ep_index + 16,
        _ => board.ep_index - 16,
    }
}
