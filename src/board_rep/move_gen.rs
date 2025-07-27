use crate::board_rep::{bit_operations::{count_bits, first_bit_pop}, board::{Board, WHITE}, c_move_list::CMoveList, magic_bitboards::MagicBitboards};

pub fn generate_moves(board: &Board, magic_bitboards: &MagicBitboards) -> CMoveList {
    let mut c_move_list = CMoveList::new();
    if board.stm == WHITE {
        let attackable_squares = board.b_occupancy | !(board.all_occupancy);

        let mut knights = board.w_knights;
        while knights != 0 {
            let square = first_bit_pop(&mut knights);
            generate_knight_moves(magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut bishops = board.w_bishops;
        while bishops != 0 {
            let square = first_bit_pop(&mut bishops);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut rooks = board.w_rooks;
        while rooks != 0 {
            let square = first_bit_pop(&mut rooks);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut queens = board.w_queens;
        while queens != 0 {
            let square = first_bit_pop(&mut queens);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut king = board.w_king;
        while king != 0 {
            let square = first_bit_pop(&mut king);
            generate_king_moves(magic_bitboards, attackable_squares, square, &mut c_move_list);
        }
    } else {
        let attackable_squares = board.w_occupancy | !(board.all_occupancy);

        let mut knights = board.b_knights;
        while knights != 0 {
            let square = first_bit_pop(&mut knights);
            generate_knight_moves(magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut bishops = board.b_bishops;
        while bishops != 0 {
            let square = first_bit_pop(&mut bishops);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut rooks = board.b_rooks;
        while rooks != 0 {
            let square = first_bit_pop(&mut rooks);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut queens = board.b_queens;
        while queens != 0 {
            let square = first_bit_pop(&mut queens);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut c_move_list);
        }

        let mut king = board.b_king;
        while king != 0 {
            let square = first_bit_pop(&mut king);
            generate_king_moves(magic_bitboards, attackable_squares, square, &mut c_move_list);
        }
    }
    c_move_list
}

fn generate_knight_moves(magic_bitboards: &MagicBitboards, attackable_squares: u64, square: u8, c_move_list: &mut CMoveList) {
    let mut knight_moves = magic_bitboards.knight_attacks[square as usize];
    knight_moves &= attackable_squares;
    while knight_moves != 0 {
        let target_square = first_bit_pop(&mut knight_moves);
        if target_square < 64 {
            c_move_list.add_move(square, target_square as u8, 0);
        }
    }
}

fn generate_bishop_moves(board: &Board, magic_bitboards: &MagicBitboards, attackable_squares: u64, square: u8, c_move_list: &mut CMoveList) {
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
            c_move_list.add_move(square, target_square as u8, 0);
        }
    }
}

fn generate_rook_moves(board: &Board, magic_bitboards: &MagicBitboards, attackable_squares: u64, square: u8, c_move_list: &mut CMoveList) {
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

fn generate_king_moves(magic_bitboards: &MagicBitboards, attackable_squares: u64, square: u8, c_move_list: &mut CMoveList) {
    let mut king_moves = magic_bitboards.king_attacks[square as usize];
    king_moves &= attackable_squares;
    while king_moves != 0 {
        let target_square = first_bit_pop(&mut king_moves);
        if target_square < 64 {
            c_move_list.add_move(square, target_square as u8, 0);
        }
    }
}