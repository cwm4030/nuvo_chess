use crate::board_state::{
    board::Board,
    c_move_list::CMoveList,
    castling::{BLACK_KING, BLACK_QUEEN, WHITE_KING, WHITE_QUEEN},
    piece_type::{
        BLACK, EMPTY_SQUARE, KING, KNIGHT, OFF_BOARD_SQUARE, PAWN, PIECE_MASK, QUEEN, WHITE,
    },
    square_index::{A8, C1, C8, G1, G8, H1},
};

const ATTACK: u8 = 1;
const DEFEND: u8 = 2;
const PIN: u8 = 4;
const EP_PIN: u8 = 8;
const RAY_DETECTION_OFFSET: i16 = -1 * (A8 as i16 - H1 as i16);
const RAY_DETECTION: [i16; 240] = [
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

pub fn generate_moves(board: &Board, c_move_list: &mut CMoveList) {
    c_move_list.clear();
    let (adp_map, check_count) = generate_adp_map(board);
    if board.stm == WHITE {
        if check_count <= 1 {
            for i in 0..board.w_pawns {
                generate_non_capture_pawn_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.w_pawn_indexes[i as usize],
                    &PAWN_WHITE_DIRECTIONS,
                    c_move_list,
                );
                generate_capture_pawn_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.w_pawn_indexes[i as usize],
                    &PAWN_WHITE_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.w_knights {
                generate_non_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.w_knight_indexes[i as usize],
                    &KNIGHT_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.w_bishops {
                generate_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.w_bishop_indexes[i as usize],
                    &BISHOP_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.w_rooks {
                generate_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.w_rook_indexes[i as usize],
                    &ROOK_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.w_queens {
                generate_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.w_queen_indexes[i as usize],
                    &QUEEN_DIRECTIONS,
                    c_move_list,
                );
            }
        }

        generate_non_sliding_moves(
            board,
            &adp_map,
            check_count,
            board.w_king_index,
            &KING_DIRECTIONS,
            c_move_list,
        );
    } else {
        if check_count <= 1 {
            for i in 0..board.b_pawns {
                generate_non_capture_pawn_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.b_pawn_indexes[i as usize],
                    &PAWN_BLACK_DIRECTIONS,
                    c_move_list,
                );
                generate_capture_pawn_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.b_pawn_indexes[i as usize],
                    &PAWN_BLACK_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.b_knights {
                generate_non_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.b_knight_indexes[i as usize],
                    &KNIGHT_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.b_bishops {
                generate_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.b_bishop_indexes[i as usize],
                    &BISHOP_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.b_rooks {
                generate_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.b_rook_indexes[i as usize],
                    &ROOK_DIRECTIONS,
                    c_move_list,
                );
            }

            for i in 0..board.b_queens {
                generate_sliding_moves(
                    board,
                    &adp_map,
                    check_count,
                    board.b_queen_indexes[i as usize],
                    &QUEEN_DIRECTIONS,
                    c_move_list,
                );
            }
        }

        generate_non_sliding_moves(
            board,
            &adp_map,
            check_count,
            board.b_king_index,
            &KING_DIRECTIONS,
            c_move_list,
        );
    }
}

fn generate_non_capture_pawn_moves(
    board: &Board,
    adp_map: &[u8; 192],
    check_count: u8,
    from_index: u8,
    directions: &[i16],
    c_move_list: &mut CMoveList,
) {
    let up_one_index = (from_index as i16 + directions[0]) as u8;
    let up_one_square = board.squares[up_one_index as usize];
    if up_one_square == EMPTY_SQUARE {
        if PAWN_PROMOTION_SQUARES[up_one_index as usize] == board.stm {
            for promotion_piece in KNIGHT..=QUEEN {
                add_legal_move(
                    board,
                    adp_map,
                    check_count,
                    from_index,
                    up_one_index,
                    promotion_piece,
                    c_move_list,
                );
            }
        } else {
            add_legal_move(
                board,
                adp_map,
                check_count,
                from_index,
                up_one_index,
                0,
                c_move_list,
            );
        }

        let up_two_index = (from_index as i16 + directions[1]) as u8;
        let up_two_square = board.squares[up_two_index as usize];
        if up_two_square == EMPTY_SQUARE && PAWN_START_SQUARES[from_index as usize] == board.stm {
            add_legal_move(
                board,
                adp_map,
                check_count,
                from_index,
                up_two_index,
                0,
                c_move_list,
            );
        }
    }
}

fn generate_capture_pawn_moves(
    board: &Board,
    adp_map: &[u8; 192],
    check_count: u8,
    from_index: u8,
    directions: &[i16],
    c_move_list: &mut CMoveList,
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
                        board,
                        adp_map,
                        check_count,
                        from_index,
                        to_index,
                        promotion_piece,
                        c_move_list,
                    );
                }
            } else {
                add_legal_move(
                    board,
                    adp_map,
                    check_count,
                    from_index,
                    to_index,
                    0,
                    c_move_list,
                );
            }
        } else if to_index == board.ep_index {
            add_legal_move(
                board,
                adp_map,
                check_count,
                from_index,
                to_index,
                0,
                c_move_list,
            );
        }
    }
}

fn generate_non_sliding_moves(
    board: &Board,
    adp_map: &[u8; 192],
    check_count: u8,
    from_index: u8,
    directions: &[i16],
    c_move_list: &mut CMoveList,
) {
    for &direction in directions {
        let to_index = (from_index as i16 + direction) as u8;
        let to_square = board.squares[to_index as usize];
        if to_square == EMPTY_SQUARE
            || to_square & OFF_BOARD_SQUARE == (board.stm ^ OFF_BOARD_SQUARE)
        {
            add_legal_move(
                board,
                adp_map,
                check_count,
                from_index,
                to_index,
                0,
                c_move_list,
            );
        }
    }
}

fn generate_sliding_moves(
    board: &Board,
    adp_map: &[u8; 192],
    check_count: u8,
    from_index: u8,
    directions: &[i16],
    c_move_list: &mut CMoveList,
) {
    for &direction in directions {
        let mut to_index = from_index as i16 + direction;
        let mut to_square = board.squares[to_index as usize];
        while to_square == EMPTY_SQUARE {
            add_legal_move(
                board,
                adp_map,
                check_count,
                from_index,
                to_index as u8,
                0,
                c_move_list,
            );
            to_index += direction;
            to_square = board.squares[to_index as usize];
        }

        if to_square & OFF_BOARD_SQUARE == (board.stm ^ OFF_BOARD_SQUARE) {
            add_legal_move(
                board,
                adp_map,
                check_count,
                from_index,
                to_index as u8,
                0,
                c_move_list,
            );
        }
    }
}

fn add_legal_move(
    board: &Board,
    adp_map: &[u8; 192],
    check_count: u8,
    from_index: u8,
    to_index: u8,
    promotion_piece: u8,
    c_move_list: &mut CMoveList,
) {
    let from_square = board.squares[from_index as usize];
    if from_square & PIECE_MASK == KING {
        if adp_map[to_index as usize] != 0 {
            return;
        } else if (is_non_sliding_attack(board, to_index)) {
            return;
        }
    } else {
        if check_count > 1 {
            return;
        } else if check_count == 1 && adp_map[to_index as usize] & DEFEND != DEFEND {
            return;
        } else if adp_map[from_index as usize] & PIN == PIN {
            return;
        } else if from_square & PIECE_MASK == PAWN
            && to_index == board.ep_index
            && adp_map[to_index as usize] & EP_PIN == EP_PIN
        {
            return;
        }
    }
    c_move_list.add_move(from_index, to_index, promotion_piece);
}

fn generate_adp_map(board: &Board) -> ([u8; 192], u8) {
    let mut adp_map = [0; 192];
    let mut check_count = 0;
    let mut dest_indexes: [u8; 3] = [0; 3];

    if board.stm == WHITE {
        dest_indexes[0] = board.w_king_index;
        if board.castling_rights & WHITE_KING != 0 {
            dest_indexes[1] = G1;
        }
        if board.castling_rights & WHITE_QUEEN != 0 {
            dest_indexes[2] = C1;
        }

        for i in 0..board.b_bishops {
            let from_index = board.b_bishop_indexes[i as usize];
            for dest_index in dest_indexes {
                if dest_index != 0 {
                    set_adp_map_ray(
                        board,
                        from_index,
                        dest_index,
                        &BISHOP_DIRECTIONS,
                        &mut adp_map,
                        &mut check_count,
                    );
                }
            }
        }

        for i in 0..board.b_rooks {
            let from_index = board.b_rook_indexes[i as usize];
            for dest_index in dest_indexes {
                if dest_index != 0 {
                    set_adp_map_ray(
                        board,
                        from_index,
                        dest_index,
                        &ROOK_DIRECTIONS,
                        &mut adp_map,
                        &mut check_count,
                    );
                }
            }
        }

        for i in 0..board.b_queens {
            let from_index = board.b_queen_indexes[i as usize];
            for dest_index in dest_indexes {
                if dest_index != 0 {
                    set_adp_map_ray(
                        board,
                        from_index,
                        dest_index,
                        &QUEEN_DIRECTIONS,
                        &mut adp_map,
                        &mut check_count,
                    );
                }
            }
        }

        for &direction in &PAWN_BLACK_DIRECTIONS[2..] {
            for dest_index in dest_indexes {
                let from_index = (dest_index as i16 + direction) as u8;
                let from_square = board.squares[from_index as usize];
                if from_square == (BLACK | PAWN) {
                    adp_map[dest_index as usize] |= ATTACK;
                    if dest_index == board.w_king_index {
                        check_count += 1;
                        adp_map[from_index as usize] |= DEFEND;
                    }
                }
            }
        }

        for &direction in &KNIGHT_DIRECTIONS {
            for dest_index in dest_indexes {
                let from_index = (dest_index as i16 + direction) as u8;
                let from_square = board.squares[from_index as usize];
                if from_square == (BLACK | KNIGHT) {
                    adp_map[dest_index as usize] |= ATTACK;
                    if dest_index == board.w_king_index {
                        check_count += 1;
                        adp_map[from_index as usize] |= DEFEND;
                    }
                }
            }
        }

        for &direction in &KING_DIRECTIONS {
            for dest_index in dest_indexes {
                let from_index = (dest_index as i16 + direction) as u8;
                let from_square = board.squares[from_index as usize];
                if from_square == (BLACK | KING) {
                    adp_map[dest_index as usize] |= ATTACK;
                    if dest_index == board.w_king_index {
                        check_count += 1;
                        adp_map[from_index as usize] |= DEFEND;
                    }
                }
            }
        }
    } else {
        dest_indexes[0] = board.b_king_index;
        if board.castling_rights & BLACK_KING != 0 {
            dest_indexes[1] = G8;
        }
        if board.castling_rights & BLACK_QUEEN != 0 {
            dest_indexes[2] = C8;
        }

        for i in 0..board.w_bishops {
            let from_index = board.w_bishop_indexes[i as usize];
            for dest_index in dest_indexes {
                if dest_index != 0 {
                    set_adp_map_ray(
                        board,
                        from_index,
                        dest_index,
                        &BISHOP_DIRECTIONS,
                        &mut adp_map,
                        &mut check_count,
                    );
                }
            }
        }

        for i in 0..board.w_rooks {
            let from_index = board.w_rook_indexes[i as usize];
            for dest_index in dest_indexes {
                if dest_index != 0 {
                    set_adp_map_ray(
                        board,
                        from_index,
                        dest_index,
                        &ROOK_DIRECTIONS,
                        &mut adp_map,
                        &mut check_count,
                    );
                }
            }
        }

        for i in 0..board.w_queens {
            let from_index = board.w_queen_indexes[i as usize];
            for dest_index in dest_indexes {
                if dest_index != 0 {
                    set_adp_map_ray(
                        board,
                        from_index,
                        dest_index,
                        &QUEEN_DIRECTIONS,
                        &mut adp_map,
                        &mut check_count,
                    );
                }
            }
        }

        for &direction in &PAWN_WHITE_DIRECTIONS[2..] {
            for dest_index in dest_indexes {
                let from_index = (dest_index as i16 + direction) as u8;
                let from_square = board.squares[from_index as usize];
                if from_square == (WHITE | PAWN) {
                    adp_map[dest_index as usize] |= ATTACK;
                    if dest_index == board.b_king_index {
                        check_count += 1;
                        adp_map[from_index as usize] |= DEFEND;
                    }
                }
            }
        }

        for &direction in &KNIGHT_DIRECTIONS {
            for dest_index in dest_indexes {
                let from_index = (dest_index as i16 + direction) as u8;
                let from_square = board.squares[from_index as usize];
                if from_square == (WHITE | KNIGHT) {
                    adp_map[dest_index as usize] |= ATTACK;
                    if dest_index == board.b_king_index {
                        check_count += 1;
                        adp_map[from_index as usize] |= DEFEND;
                    }
                }
            }
        }

        for &direction in &KING_DIRECTIONS {
            for dest_index in dest_indexes {
                let from_index = (dest_index as i16 + direction) as u8;
                let from_square = board.squares[from_index as usize];
                if from_square == (WHITE | KING) {
                    adp_map[dest_index as usize] |= ATTACK;
                    if dest_index == board.b_king_index {
                        check_count += 1;
                        adp_map[from_index as usize] |= DEFEND;
                    }
                }
            }
        }
    }

    (adp_map, check_count)
}

fn set_adp_map_ray(
    board: &Board,
    from_index: u8,
    dest_index: u8,
    directions: &[i16],
    adp_map: &mut [u8; 192],
    check_count: &mut u8,
) {
    let attack_direction = get_attack_direction(from_index, dest_index, directions);
    if attack_direction != 0 {
        let dest_is_king_index = match board.stm {
            WHITE => board.w_king_index == dest_index,
            _ => board.b_king_index == dest_index,
        };
        let to_index = get_attacked_square_index(board, from_index, attack_direction);
        let to_square = board.squares[to_index as usize];
        if to_index == dest_index {
            if dest_is_king_index {
                *check_count += 1;
                let mut adp_index = from_index;
                while adp_index != to_index {
                    adp_map[adp_index as usize] |= ATTACK | DEFEND;
                    adp_index = (adp_index as i16 + attack_direction) as u8;
                }
                adp_index = (adp_index as i16 + attack_direction) as u8; // skip the king square
                adp_map[adp_index as usize] |= ATTACK; // extra square past the king
            } else {
                adp_map[to_index as usize] |= ATTACK;
            }
        } else if dest_is_king_index && to_square & OFF_BOARD_SQUARE == board.stm {
            let next_to_index = get_attacked_square_index(board, to_index, attack_direction);
            if next_to_index == dest_index {
                adp_map[to_index as usize] |= PIN;
            }
        } else if dest_is_king_index
            && to_square & PIECE_MASK == PAWN
            && (attack_direction == -1 || attack_direction == 1)
        {
            let ep_pawn_index = get_ep_pawn_index(board);
            let next_to_index = (to_index as i16 + attack_direction) as u8;
            let next_to_square = board.squares[next_to_index as usize];
            if next_to_square & PIECE_MASK == PAWN
                && (to_square | next_to_square) & OFF_BOARD_SQUARE == OFF_BOARD_SQUARE
                && (to_square == ep_pawn_index || next_to_index == ep_pawn_index)
            {
                let next_next_to_index =
                    get_attacked_square_index(board, next_to_index, attack_direction);
                if next_next_to_index == dest_index {
                    adp_map[board.ep_index as usize] |= EP_PIN;
                }
            }
        }
    }
}

fn get_attack_direction(from_index: u8, to_index: u8, directions: &[i16]) -> i16 {
    let ray_dection_index = from_index as i16 - to_index as i16 + RAY_DETECTION_OFFSET;
    let ray_detection = RAY_DETECTION[ray_dection_index as usize];
    if ray_detection == 0 {
        return 0;
    }

    for &direction in directions {
        if ray_detection == direction {
            return direction;
        }
    }
    0
}

fn get_attacked_square_index(board: &Board, from_index: u8, direction: i16) -> u8 {
    let mut to_index = from_index as i16 + direction;
    let mut to_square = board.squares[to_index as usize];
    while to_square == EMPTY_SQUARE {
        to_index += direction;
        to_square = board.squares[to_index as usize];
    }
    to_index as u8
}

fn get_ep_pawn_index(board: &Board) -> u8 {
    if board.ep_index == 0 {
        return 0;
    }

    return match board.stm {
        WHITE => board.ep_index + 16,
        _ => board.ep_index - 16,
    };
}

fn is_non_sliding_attack(board: &Board, index: u8) -> bool {
    if board.stm == WHITE {
        for &direction in &PAWN_BLACK_DIRECTIONS[2..] {
            let attack_index = (index as i16 + direction) as u8;
            let attack_square = board.squares[attack_index as usize];
            if attack_square == (BLACK | PAWN) {
                return true;
            }
        }

        for &direction in &KNIGHT_DIRECTIONS[2..] {
            let attack_index = (index as i16 + direction) as u8;
            let attack_square = board.squares[attack_index as usize];
            if attack_square == (BLACK | KNIGHT) {
                return true;
            }
        }

        for &direction in &KING_DIRECTIONS[2..] {
            let attack_index = (index as i16 + direction) as u8;
            let attack_square = board.squares[attack_index as usize];
            if attack_square == (BLACK | KING) {
                return true;
            }
        }
    } else {
        for &direction in &PAWN_WHITE_DIRECTIONS[2..] {
            let attack_index = (index as i16 + direction) as u8;
            let attack_square = board.squares[attack_index as usize];
            if attack_square == (WHITE | PAWN) {
                return true;
            }
        }

        for &direction in &KNIGHT_DIRECTIONS[2..] {
            let attack_index = (index as i16 + direction) as u8;
            let attack_square = board.squares[attack_index as usize];
            if attack_square == (WHITE | KNIGHT) {
                return true;
            }
        }

        for &direction in &KING_DIRECTIONS[2..] {
            let attack_index = (index as i16 + direction) as u8;
            let attack_square = board.squares[attack_index as usize];
            if attack_square == (WHITE | KING) {
                return true;
            }
        }
    }
    false
}
