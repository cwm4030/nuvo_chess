use crate::board_state::c_move::CMove;
use crate::board_state::piece_type::{BISHOP, KNIGHT, QUEEN, ROOK};
use crate::board_state::square_index::SQUARE_NAMES;

pub struct CMoveList {
    pub moves: [CMove; 256],
    pub count: usize,
}

impl CMoveList {
    pub fn new() -> Self {
        CMoveList {
            moves: [CMove {
                from_index: 0,
                to_index: 0,
                promotion_piece: 0,
            }; 256],
            count: 0,
        }
    }

    pub fn add_move(&mut self, from_index: u8, to_index: u8, promotion_piece: u8) {
        self.moves[self.count] = CMove {
            from_index,
            to_index,
            promotion_piece,
        };
        self.count += 1;
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }

    pub fn print_moves(&self) {
        for i in 0..self.count {
            let from_square = SQUARE_NAMES[self.moves[i].from_index as usize];
            let to_square = SQUARE_NAMES[self.moves[i].to_index as usize];
            let promotion_piece = match self.moves[i].promotion_piece {
                KNIGHT => "N",
                BISHOP => "B",
                ROOK => "R",
                QUEEN => "Q",
                _ => "",
            };
            println!("{}) {}{}{}", i + 1, from_square, to_square, promotion_piece);
        }
        println!();
    }
}
