use crate::board_state::{
    piece_type::{BISHOP, BLACK, KNIGHT, QUEEN, ROOK, WHITE},
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

    pub fn get_c_move_string(&self, stm: u8) -> String {
        let from_square = SQUARE_NAMES[self.from_index as usize];
        let to_square = SQUARE_NAMES[self.to_index as usize];
        let promotion_piece = match stm | self.promotion_piece {
            x if x == (BLACK | KNIGHT) => "n",
            x if x == (BLACK | BISHOP) => "b",
            x if x == (BLACK | ROOK) => "r",
            x if x == (BLACK | QUEEN) => "q",
            x if x == (WHITE | KNIGHT) => "N",
            x if x == (WHITE | BISHOP) => "B",
            x if x == (WHITE | ROOK) => "R",
            x if x == (WHITE | QUEEN) => "Q",
            _ => "",
        };
        if self.promotion_piece != 0 {
            format!("{}{}{}", from_square, to_square, promotion_piece)
        } else {
            format!("{}{}", from_square, to_square)
        }
    }
}
