pub const EMPTY_SQUARE: u8 = 0;
pub const OFF_BOARD_SQUARE: u8 = 1;
pub const PAWN: u8 = 2;
pub const KNIGHT: u8 = 4;
pub const BISHOP: u8 = 6;
pub const ROOK: u8 = 8;
pub const QUEEN: u8 = 10;
pub const KING: u8 = 12;
pub const WHITE: u8 = 16;
pub const BLACK: u8 = 32;
pub const CAPTURE: u8 = 64;
pub const PIECE_MASK: u8 = 14;
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