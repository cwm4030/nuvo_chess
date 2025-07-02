use crate::board_state::{
    piece_type::{BISHOP, KNIGHT, QUEEN, ROOK},
    square_index::SQUARE_NAMES,
};

#[derive(Copy, Clone)]
pub struct CMove {
    pub from_index: u8,
    pub to_index: u8,
    pub promotion_piece: u8,
}

impl CMove {
    pub fn new() -> Self {
        CMove {
            from_index: 0,
            to_index: 0,
            promotion_piece: 0,
        }
    }

    pub fn get_c_move_string(&self) -> String {
        let from_square = SQUARE_NAMES[self.from_index as usize];
        let to_square = SQUARE_NAMES[self.to_index as usize];
        let promotion_piece = match self.promotion_piece {
            KNIGHT => "n",
            BISHOP => "b",
            ROOK => "r",
            QUEEN => "q",
            _ => "",
        };
        if self.promotion_piece != 0 {
            format!("{from_square}{to_square}{promotion_piece}")
        } else {
            format!("{from_square}{to_square}")
        }
    }
}
