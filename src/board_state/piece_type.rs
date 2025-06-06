pub const EMPTY_SQUARE: u8 = 0;
pub const OFF_BOARD_SQUARE: u8 = 1;
pub const PAWN: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;
pub const WHITE: u8 = 8;
pub const BLACK: u8 = 16;
pub const CAPTURE: u8 = 32;
pub const PIECE_MASK: u8 = 7;
pub const COLOR_MASK: u8 = 48;

pub fn is_king(piece_type: u8) -> bool {
    (piece_type & PIECE_MASK) == KING
}

pub fn is_white(piece_type: u8) -> bool {
    (piece_type & WHITE) != 0
}

pub fn is_black(piece_type: u8) -> bool {
    (piece_type & BLACK) != 0
}

pub fn get_piece_string(piece_type: u8, use_ascii_piece: bool) -> String {
    if use_ascii_piece {
        match piece_type {
            EMPTY_SQUARE => " . ",
            x if x == (PAWN | BLACK) => " p ",
            x if x == (KNIGHT | BLACK) => " n ",
            x if x == (BISHOP | BLACK) => " b ",
            x if x == (ROOK | BLACK) => " r ",
            x if x == (QUEEN | BLACK) => " q ",
            x if x == (KING | BLACK) => " k ",
            x if x == (PAWN | WHITE) => " P ",
            x if x == (KNIGHT | WHITE) => " N ",
            x if x == (BISHOP | WHITE) => " B ",
            x if x == (ROOK | WHITE) => " R ",
            x if x == (QUEEN | WHITE) => " Q ",
            x if x == (KING | WHITE) => " K ",
            _ => " ? ",
        }
        .to_string()
    } else {
        match piece_type & PIECE_MASK {
            EMPTY_SQUARE => "   ",
            PAWN => " ♟ ",
            KNIGHT => " ♞ ",
            BISHOP => " ♝ ",
            ROOK => " ♜ ",
            QUEEN => " ♛ ",
            KING => " ♚ ",
            _ => " ? ",
        }
        .to_string()
    }
}
