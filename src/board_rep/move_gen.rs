use crate::board_rep::{
    bit_operations::{count_bits, first_bit, first_bit_pop, set_bit},
    board::{Board, EMPTY, EN_PASSANT_NONE, KING, KNIGHT, PAWN, PIECE_MASK, WHITE},
    c_move::CMove,
    c_move_list::CMoveList,
    magic_bitboards::MagicBitboards,
    squares::{H2, H8},
};

const PINNER: u8 = 0;
const PIN: u8 = 16;
const EP_PIN: u8 = 32;
const DEFEND: u8 = 64;

pub struct MoveInfo {
    pub c_move_list: CMoveList,
    pub check_count: u8,
    pub pin_defend_map: [u8; 64],
}

impl Default for MoveInfo {
    fn default() -> Self {
        MoveInfo::new()
    }
}

impl MoveInfo {
    pub fn new() -> Self {
        MoveInfo {
            c_move_list: CMoveList::new(),
            check_count: 0,
            pin_defend_map: [0; 64],
        }
    }

    pub fn get_legal_move_count(&self, board: &Board, magic_bitboards: &MagicBitboards) -> usize {
        let mut legal_moves = 0;
        for i in 0..self.c_move_list.count {
            let c_move = self.c_move_list.moves[i];
            if self.is_move_legal(board, magic_bitboards, &c_move) {
                legal_moves += 1;
            }
        }
        legal_moves
    }

    pub fn is_move_legal(
        &self,
        board: &Board,
        magic_bitboards: &MagicBitboards,
        c_move: &CMove,
    ) -> bool {
        let from_square = board.get_piece_at(c_move.from_square);
        if from_square & PIECE_MASK == KING {
            if is_square_attacked(board, magic_bitboards, c_move.to_square) {
                return false;
            }
        } else if self.check_count > 1 {
            return false;
        } else if self.check_count == 1 {
            if (c_move.to_square == board.en_passant
                && board.en_passant != EN_PASSANT_NONE
                && self.pin_defend_map[board.en_passant as usize] & DEFEND == DEFEND
                && from_square & PIECE_MASK != PAWN)
                || self.pin_defend_map[c_move.to_square as usize] & DEFEND != DEFEND
                || self.pin_defend_map[c_move.from_square as usize] & PIN == PIN
            {
                return false;
            }
        } else if (self.pin_defend_map[c_move.from_square as usize] & PIN == PIN
            && (self.pin_defend_map[c_move.from_square as usize] & PINNER
                != self.pin_defend_map[c_move.to_square as usize] & PINNER))
            || (from_square & PIECE_MASK == PAWN
                && c_move.to_square == board.en_passant
                && self.pin_defend_map[c_move.to_square as usize] & EP_PIN == EP_PIN)
        {
            return false;
        }
        true
    }
}

pub fn generate_moves(board: &Board, magic_bitboards: &MagicBitboards) -> MoveInfo {
    let mut mi = generate_move_info(board, magic_bitboards);
    if board.stm == WHITE {
        let empty_squares = !(board.all_occupancy);
        let opponent_occupancy = board.b_occupancy;
        let attackable_squares = board.b_occupancy | empty_squares;

        let mut pawns = board.w_pawns;
        while pawns != 0 {
            let square = first_bit_pop(&mut pawns);
            generate_w_pawn_moves(
                board,
                magic_bitboards,
                opponent_occupancy,
                empty_squares,
                square,
                &mut mi,
            );
        }

        let mut knights = board.w_knights;
        while knights != 0 {
            let square = first_bit_pop(&mut knights);
            generate_knight_moves(magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut bishops = board.w_bishops;
        while bishops != 0 {
            let square = first_bit_pop(&mut bishops);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut rooks = board.w_rooks;
        while rooks != 0 {
            let square = first_bit_pop(&mut rooks);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut queens = board.w_queens;
        while queens != 0 {
            let square = first_bit_pop(&mut queens);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut king = board.w_king;
        while king != 0 {
            let square = first_bit_pop(&mut king);
            generate_king_moves(magic_bitboards, attackable_squares, square, &mut mi);
        }
    } else {
        let empty_squares = !(board.all_occupancy);
        let opponent_occupancy = board.w_occupancy;
        let attackable_squares = board.w_occupancy | empty_squares;

        let mut pawns = board.b_pawns;
        while pawns != 0 {
            let square = first_bit_pop(&mut pawns);
            generate_b_pawn_moves(
                board,
                magic_bitboards,
                opponent_occupancy,
                empty_squares,
                square,
                &mut mi,
            );
        }

        let mut knights = board.b_knights;
        while knights != 0 {
            let square = first_bit_pop(&mut knights);
            generate_knight_moves(magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut bishops = board.b_bishops;
        while bishops != 0 {
            let square = first_bit_pop(&mut bishops);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut rooks = board.b_rooks;
        while rooks != 0 {
            let square = first_bit_pop(&mut rooks);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut queens = board.b_queens;
        while queens != 0 {
            let square = first_bit_pop(&mut queens);
            generate_bishop_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
            generate_rook_moves(board, magic_bitboards, attackable_squares, square, &mut mi);
        }

        let mut king = board.b_king;
        while king != 0 {
            let square = first_bit_pop(&mut king);
            generate_king_moves(magic_bitboards, attackable_squares, square, &mut mi);
        }
    }
    mi
}

fn generate_w_pawn_moves(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    opponent_occupancy: u64,
    empty_squares: u64,
    square: u8,
    mi: &mut MoveInfo,
) {
    let mut non_capture_pawn_moves = magic_bitboards.w_pawn_non_capture[square as usize];
    non_capture_pawn_moves &= empty_squares;

    let mut capture_pawn_moves = magic_bitboards.w_pawn_capture[square as usize];
    capture_pawn_moves &= opponent_occupancy | set_bit(0, board.en_passant);

    let mut pawn_moves = non_capture_pawn_moves | capture_pawn_moves;
    while pawn_moves != 0 {
        let target_square = first_bit_pop(&mut pawn_moves);
        let rank_diff = ((target_square / 8) as i8 - (square / 8) as i8).abs();
        if target_square <= H8 {
            for promotion in KNIGHT..KING {
                mi.c_move_list.add_move(square, target_square, promotion);
            }
        } else if rank_diff == 2 {
            let up_one_piece = board.get_piece_at(target_square + 8);
            if up_one_piece & PIECE_MASK == EMPTY {
                mi.c_move_list.add_move(square, target_square, 0);
            }
        } else {
            mi.c_move_list.add_move(square, target_square, 0);
        }
    }
}

fn generate_b_pawn_moves(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    opponent_occupancy: u64,
    empty_squares: u64,
    square: u8,
    mi: &mut MoveInfo,
) {
    let mut non_capture_pawn_moves = magic_bitboards.b_pawn_non_capture[square as usize];
    non_capture_pawn_moves &= empty_squares;

    let mut capture_pawn_moves = magic_bitboards.b_pawn_capture[square as usize];
    capture_pawn_moves &= opponent_occupancy | set_bit(0, board.en_passant);

    let mut pawn_moves = non_capture_pawn_moves | capture_pawn_moves;
    while pawn_moves != 0 {
        let target_square = first_bit_pop(&mut pawn_moves);
        let rank_diff = ((target_square / 8) as i8 - (square / 8) as i8).abs();
        if target_square > H2 {
            for promotion in KNIGHT..KING {
                mi.c_move_list.add_move(square, target_square, promotion);
            }
        } else if rank_diff == 2 {
            let up_one_piece = board.get_piece_at(target_square - 8);
            if up_one_piece & PIECE_MASK == EMPTY {
                mi.c_move_list.add_move(square, target_square, 0);
            }
        } else {
            mi.c_move_list.add_move(square, target_square, 0);
        }
    }
}

fn generate_knight_moves(
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    mi: &mut MoveInfo,
) {
    let mut knight_moves = magic_bitboards.knight_attacks[square as usize];
    knight_moves &= attackable_squares;
    while knight_moves != 0 {
        let target_square = first_bit_pop(&mut knight_moves);
        mi.c_move_list.add_move(square, target_square, 0);
    }
}

fn generate_bishop_moves(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    mi: &mut MoveInfo,
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
        mi.c_move_list.add_move(square, target_square, 0);
    }
}

fn generate_rook_moves(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    mi: &mut MoveInfo,
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
        mi.c_move_list.add_move(square, target_square as u8, 0);
    }
}

fn generate_king_moves(
    magic_bitboards: &MagicBitboards,
    attackable_squares: u64,
    square: u8,
    mi: &mut MoveInfo,
) {
    let mut king_moves = magic_bitboards.king_attacks[square as usize];
    king_moves &= attackable_squares;
    while king_moves != 0 {
        let target_square = first_bit_pop(&mut king_moves);
        mi.c_move_list.add_move(square, target_square, 0);
    }
}

fn is_square_attacked(board: &Board, magic_bitboards: &MagicBitboards, square: u8) -> bool {
    let stm_king: u64;
    let opponent_kings: u64;
    let opponent_rook_queens: u64;
    let opponent_bishop_queens: u64;
    let opponent_knights: u64;
    let opponent_pawns: u64;
    if board.stm == WHITE {
        stm_king = board.w_king;
        opponent_kings = board.b_king;
        opponent_rook_queens = board.b_rooks | board.b_queens;
        opponent_bishop_queens = board.b_bishops | board.b_queens;
        opponent_knights = board.b_knights;
        opponent_pawns = board.b_pawns;
    } else {
        stm_king = board.b_king;
        opponent_kings = board.w_king;
        opponent_rook_queens = board.w_rooks | board.w_queens;
        opponent_bishop_queens = board.w_bishops | board.w_queens;
        opponent_knights = board.w_knights;
        opponent_pawns = board.w_pawns;
    }

    let king_attacks = magic_bitboards.king_attacks[square as usize];
    if king_attacks & opponent_kings != 0 {
        return true;
    }

    let rook_mask = magic_bitboards.rook_masks[square as usize];
    let rook_mask_bits = count_bits(rook_mask);
    let rook_key = rook_mask & (board.all_occupancy ^ stm_king);
    let rook_magic = magic_bitboards.rook_magics[square as usize];
    let rook_index = MagicBitboards::magic_function(rook_key, rook_magic, rook_mask_bits as u8);
    let rook_attacks = magic_bitboards.rook_attacks[square as usize][rook_index as usize];
    if rook_attacks & opponent_rook_queens != 0 {
        return true;
    }

    let bishop_mask = magic_bitboards.bishop_masks[square as usize];
    let bishop_mask_bits = count_bits(bishop_mask);
    let bishop_key = bishop_mask & (board.all_occupancy ^ stm_king);
    let bishop_magic = magic_bitboards.bishop_magics[square as usize];
    let bishop_index =
        MagicBitboards::magic_function(bishop_key, bishop_magic, bishop_mask_bits as u8);
    let bishop_attacks = magic_bitboards.bishop_attacks[square as usize][bishop_index as usize];
    if bishop_attacks & opponent_bishop_queens != 0 {
        return true;
    }

    let knight_attacks = magic_bitboards.knight_attacks[square as usize];
    if knight_attacks & opponent_knights != 0 {
        return true;
    }

    let pawn_attacks = if board.stm == WHITE {
        magic_bitboards.w_pawn_capture[square as usize]
    } else {
        magic_bitboards.b_pawn_capture[square as usize]
    };
    if pawn_attacks & opponent_pawns != 0 {
        return true;
    }
    false
}

fn generate_move_info(board: &Board, magic_bitboards: &MagicBitboards) -> MoveInfo {
    let stm_king_square: u64;
    let stm_kings: u64;
    let stm_occupancy: u64;
    let mut pawn_attacks: u64;
    let ep_pin_squares = get_ep_pin_squares(board);
    let opponent_rook_queens: u64;
    let opponent_bishop_queens: u64;
    let opponent_knights: u64;
    let opponent_pawns: u64;
    if board.stm == WHITE {
        stm_king_square = first_bit(board.w_king) as u64;
        stm_kings = board.w_king;
        stm_occupancy = board.w_occupancy;
        pawn_attacks = magic_bitboards.w_pawn_capture[stm_king_square as usize];
        opponent_rook_queens = board.b_rooks | board.b_queens;
        opponent_bishop_queens = board.b_bishops | board.b_queens;
        opponent_knights = board.b_knights;
        opponent_pawns = board.b_pawns;
    } else {
        stm_king_square = first_bit(board.b_king) as u64;
        stm_kings = board.b_king;
        stm_occupancy = board.b_occupancy;
        pawn_attacks = magic_bitboards.b_pawn_capture[stm_king_square as usize];
        opponent_rook_queens = board.w_rooks | board.w_queens;
        opponent_bishop_queens = board.w_bishops | board.w_queens;
        opponent_knights = board.w_knights;
        opponent_pawns = board.w_pawns;
    }
    let mut mi = MoveInfo::new();
    let mut pinner: u8 = 1;

    pawn_attacks &= opponent_pawns;
    if pawn_attacks != 0 {
        mi.check_count += count_bits(pawn_attacks) as u8;
        while pawn_attacks != 0 {
            let target_square = first_bit_pop(&mut pawn_attacks);
            mi.pin_defend_map[target_square as usize] |= DEFEND;
        }
    }
    let ep_pawn_index = board.get_ep_pawn_index();
    if ep_pawn_index != EN_PASSANT_NONE
        && mi.pin_defend_map[ep_pawn_index as usize] & DEFEND == DEFEND
    {
        mi.pin_defend_map[ep_pawn_index as usize] |= DEFEND;
    }

    let mut knight_attacks = magic_bitboards.knight_attacks[stm_king_square as usize];
    knight_attacks &= opponent_knights;
    if knight_attacks != 0 {
        mi.check_count += count_bits(knight_attacks) as u8;
        while knight_attacks != 0 {
            let target_square = first_bit_pop(&mut knight_attacks);
            mi.pin_defend_map[target_square as usize] |= DEFEND;
        }
    }

    let rook_mask = magic_bitboards.rook_masks[stm_king_square as usize];
    let rook_mask_bits = count_bits(rook_mask);
    let rook_key = rook_mask & (board.all_occupancy ^ stm_kings);
    let rook_magic = magic_bitboards.rook_magics[stm_king_square as usize];
    let rook_index = MagicBitboards::magic_function(rook_key, rook_magic, rook_mask_bits as u8);
    let rook_attacks = magic_bitboards.rook_attacks[stm_king_square as usize][rook_index as usize];
    let mut rook_attackers = rook_attacks & opponent_rook_queens;
    let ep_possible_pin = rook_attacks & ep_pin_squares;
    let mut possible_pins = rook_attacks & stm_occupancy;
    if rook_attackers != 0 {
        mi.check_count += count_bits(rook_attackers) as u8;
        while rook_attackers != 0 {
            let target_square = first_bit_pop(&mut rook_attackers);
            mi.pin_defend_map[target_square as usize] |= DEFEND;
        }
    }
    if ep_possible_pin != 0 {
        let rook_key = rook_mask & (board.all_occupancy ^ (stm_kings | ep_possible_pin));
        let rook_index = MagicBitboards::magic_function(rook_key, rook_magic, rook_mask_bits as u8);
        let rook_attacks = magic_bitboards.rook_attacks[stm_king_square as usize][rook_index as usize];
        let mut pin_squares = rook_attacks & opponent_rook_queens;
        if pin_squares != 0 {
            mi.pin_defend_map[board.en_passant as usize] |= EP_PIN;
            while pin_squares != 0 {
                let pin_square = first_bit_pop(&mut pin_squares);
                mi.pin_defend_map[pin_square as usize] |= pinner;
            }
            pinner += 1;
        }
    }
    if possible_pins != 0 {
        while possible_pins != 0 {
            let possible_pin = first_bit_pop(&mut possible_pins) as u64;
            let rook_key = rook_mask & (board.all_occupancy ^ (stm_kings | possible_pin));
            let rook_index =
                MagicBitboards::magic_function(rook_key, rook_magic, rook_mask_bits as u8);
            let rook_attacks = magic_bitboards.rook_attacks[stm_king_square as usize][rook_index as usize];
            let mut pin_squares = rook_attacks & opponent_rook_queens;
            if pin_squares != 0 {
                while pin_squares != 0 {
                    let pin_square = first_bit_pop(&mut pin_squares);
                    mi.pin_defend_map[pin_square as usize] |= pinner;
                }
                pinner += 1;
            }
        }
    }

    let bishop_mask = magic_bitboards.bishop_masks[stm_king_square as usize];
    let bishop_mask_bits = count_bits(bishop_mask);
    let bishop_key = bishop_mask & (board.all_occupancy ^ stm_kings);
    let bishop_magic = magic_bitboards.bishop_magics[stm_king_square as usize];
    let bishop_index =
        MagicBitboards::magic_function(bishop_key, bishop_magic, bishop_mask_bits as u8);
    let bishop_attacks = magic_bitboards.bishop_attacks[stm_king_square as usize][bishop_index as usize];
    let mut bishop_attackers = bishop_attacks & opponent_bishop_queens;
    let mut possible_pins = bishop_attacks & stm_occupancy;
    if bishop_attackers != 0 {
        mi.check_count += count_bits(bishop_attackers) as u8;
        while bishop_attackers != 0 {
            let target_square = first_bit_pop(&mut bishop_attackers);
            mi.pin_defend_map[target_square as usize] |= DEFEND;
        }
    }
    if possible_pins != 0 {
        while possible_pins != 0 {
            let possible_pin_square = first_bit_pop(&mut possible_pins);
            let possible_pin = set_bit(0, possible_pin_square);
            let bishop_key = bishop_mask & (board.all_occupancy ^ (stm_kings | possible_pin));
            let bishop_index =
                MagicBitboards::magic_function(bishop_key, bishop_magic, bishop_mask_bits as u8);
            let mut bishop_attacks =
                magic_bitboards.bishop_attacks[stm_king_square as usize][bishop_index as usize];
            let is_pin = (bishop_attacks & opponent_bishop_queens) != 0;
            if is_pin {
                while bishop_attacks != 0 {
                    let pin_square = first_bit_pop(&mut bishop_attacks);
                    mi.pin_defend_map[pin_square as usize] |= pinner;
                }
                pinner += 1;
            }
        }
    }

    mi
}

fn get_ep_pin_squares(board: &Board) -> u64 {
    if board.en_passant == EN_PASSANT_NONE {
        return 0;
    }

    let file = (board.en_passant % 8) as i8;
    let rank = (board.en_passant / 8) as i8 + if board.stm == WHITE { -1 } else { 1 };

    let mut ep_pin_squares = 0;
    let ep_pawn_file = file;
    let left_pawn_file = file - 1;
    let right_pawn_file = file + 1;

    ep_pin_squares = set_bit(ep_pin_squares, (rank * 8 + ep_pawn_file) as u8);
    if (0..8).contains(&left_pawn_file) {
        ep_pin_squares = set_bit(ep_pin_squares, (rank * 8 + left_pawn_file) as u8);
    }
    if (0..8).contains(&right_pawn_file) {
        ep_pin_squares = set_bit(ep_pin_squares, (rank * 8 + right_pawn_file) as u8);
    }

    ep_pin_squares &= board.w_pawns | board.b_pawns;
    if count_bits(ep_pin_squares) == 2 {
        ep_pin_squares
    } else {
        0
    }
}
