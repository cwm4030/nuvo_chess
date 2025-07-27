use crate::board_rep::{
    bit_operations::{count_bits, first_bit_pop},
    board::{Board, KING, KNIGHT, WHITE},
    c_move_list::CMoveList,
    magic_bitboards::MagicBitboards,
    squares::{H2, H8},
};

pub fn generate_moves(board: &Board, magic_bitboards: &MagicBitboards) -> CMoveList {
    let mut c_move_list = CMoveList::new();
    if board.stm == WHITE {
        let empty_squares = !(board.all_occupancy);
        let opponent_occupancy = board.b_occupancy;
        let attackable_squares = board.b_occupancy | empty_squares;

        let mut pawns = board.w_pawns;
        while pawns != 0 {
            let square = first_bit_pop(&mut pawns);
            generate_w_pawn_moves(
                magic_bitboards,
                opponent_occupancy,
                empty_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut knights = board.w_knights;
        while knights != 0 {
            let square = first_bit_pop(&mut knights);
            generate_knight_moves(
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut bishops = board.w_bishops;
        while bishops != 0 {
            let square = first_bit_pop(&mut bishops);
            generate_bishop_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut rooks = board.w_rooks;
        while rooks != 0 {
            let square = first_bit_pop(&mut rooks);
            generate_rook_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut queens = board.w_queens;
        while queens != 0 {
            let square = first_bit_pop(&mut queens);
            generate_bishop_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
            generate_rook_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut king = board.w_king;
        while king != 0 {
            let square = first_bit_pop(&mut king);
            generate_king_moves(
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }
    } else {
        let empty_squares = !(board.all_occupancy);
        let opponent_occupancy = board.w_occupancy;
        let attackable_squares = board.w_occupancy | empty_squares;

        let mut pawns = board.b_pawns;
        while pawns != 0 {
            let square = first_bit_pop(&mut pawns);
            generate_b_pawn_moves(
                magic_bitboards,
                opponent_occupancy,
                empty_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut knights = board.b_knights;
        while knights != 0 {
            let square = first_bit_pop(&mut knights);
            generate_knight_moves(
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut bishops = board.b_bishops;
        while bishops != 0 {
            let square = first_bit_pop(&mut bishops);
            generate_bishop_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut rooks = board.b_rooks;
        while rooks != 0 {
            let square = first_bit_pop(&mut rooks);
            generate_rook_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut queens = board.b_queens;
        while queens != 0 {
            let square = first_bit_pop(&mut queens);
            generate_bishop_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
            generate_rook_moves(
                board,
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }

        let mut king = board.b_king;
        while king != 0 {
            let square = first_bit_pop(&mut king);
            generate_king_moves(
                magic_bitboards,
                attackable_squares,
                square,
                &mut c_move_list,
            );
        }
    }
    c_move_list
}

fn generate_w_pawn_moves(
    magic_bitboards: &MagicBitboards,
    opponent_occupancy: u64,
    empty_squares: u64,
    square: u8,
    c_move_list: &mut CMoveList,
) {
    let mut non_capture_pawn_moves = magic_bitboards.w_pawn_non_capture[square as usize];
    non_capture_pawn_moves &= empty_squares;

    let mut capture_pawn_moves = magic_bitboards.w_pawn_capture[square as usize];
    capture_pawn_moves &= opponent_occupancy;

    let mut pawn_moves = non_capture_pawn_moves | capture_pawn_moves;
    while pawn_moves != 0 {
        let target_square = first_bit_pop(&mut pawn_moves);
        if target_square <= H8 {
            for promotion in KNIGHT..KING {
                c_move_list.add_move(square, target_square, promotion);
            }
        } else {
            c_move_list.add_move(square, target_square, 0);
        }
    }
}

fn generate_b_pawn_moves(
    magic_bitboards: &MagicBitboards,
    opponent_occupancy: u64,
    empty_squares: u64,
    square: u8,
    c_move_list: &mut CMoveList,
) {
    let mut non_capture_pawn_moves = magic_bitboards.b_pawn_non_capture[square as usize];
    non_capture_pawn_moves &= empty_squares;

    let mut capture_pawn_moves = magic_bitboards.b_pawn_capture[square as usize];
    capture_pawn_moves &= opponent_occupancy;

    let mut pawn_moves = non_capture_pawn_moves | capture_pawn_moves;
    while pawn_moves != 0 {
        let target_square = first_bit_pop(&mut pawn_moves);
        if target_square > H2 {
            for promotion in KNIGHT..KING {
                c_move_list.add_move(square, target_square, promotion);
            }
        } else {
            c_move_list.add_move(square, target_square, 0);
        }
    }
}

fn generate_knight_moves(
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    c_move_list: &mut CMoveList,
) {
    let mut knight_moves = magic_bitboards.knight_attacks[square as usize];
    knight_moves &= attackable_squares;
    while knight_moves != 0 {
        let target_square = first_bit_pop(&mut knight_moves);
        if target_square < 64 {
            c_move_list.add_move(square, target_square, 0);
        }
    }
}

fn generate_bishop_moves(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    c_move_list: &mut CMoveList,
) {
    let mask = magic_bitboards.bishop_masks[square as usize];
    let mask_bits = count_bits(mask);
    let key = mask & board.all_occupancy;
    let magic = magic_bitboards.bishop_magics[square as usize];
    let index = MagicBitboards::magic_function(key, magic, mask_bits as u8);
    let mut bishop_moves = magic_bitboards.bishop_attacks[square as usize][index as usize];
    bishop_moves &= attackable_squares;
    while bishop_moves != 0 {
        let target_square = first_bit_pop(&mut bishop_moves);
        if target_square < 64 {
            c_move_list.add_move(square, target_square, 0);
        }
    }
}

fn generate_rook_moves(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    c_move_list: &mut CMoveList,
) {
    let mask = magic_bitboards.rook_masks[square as usize];
    let mask_bits = count_bits(mask);
    let key = mask & board.all_occupancy;
    let magic = magic_bitboards.rook_magics[square as usize];
    let index = MagicBitboards::magic_function(key, magic, mask_bits as u8);
    let mut rook_moves = magic_bitboards.rook_attacks[square as usize][index as usize];
    rook_moves &= attackable_squares;
    while rook_moves != 0 {
        let target_square = first_bit_pop(&mut rook_moves);
        if target_square < 64 {
            c_move_list.add_move(square, target_square as u8, 0);
        }
    }
}

fn generate_king_moves(
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    c_move_list: &mut CMoveList,
) {
    let mut king_moves = magic_bitboards.king_attacks[square as usize];
    king_moves &= attackable_squares;
    while king_moves != 0 {
        let target_square = first_bit_pop(&mut king_moves);
        if target_square < 64 {
            c_move_list.add_move(square, target_square, 0);
        }
    }
}
