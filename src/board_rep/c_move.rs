use std::fmt::{Display, Formatter, Result};

use crate::board_rep::{
    board::{BISHOP, KNIGHT, QUEEN, ROOK},
    squares::SQUARE_NAMES,
};

#[derive(Clone, Copy, Default)]
pub struct CMove {
    pub from_square: u8,
    pub to_square: u8,
    pub promotion: u8,
}

impl Display for CMove {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let mut move_str = String::new();
        move_str.push_str(SQUARE_NAMES[self.from_square as usize]);
        move_str.push_str(SQUARE_NAMES[self.to_square as usize]);
        if self.promotion != 0 {
            match self.promotion {
                KNIGHT => move_str.push('n'),
                BISHOP => move_str.push('b'),
                ROOK => move_str.push('r'),
                QUEEN => move_str.push('q'),
                _ => {}
            }
        }
        write!(f, "{move_str}")
    }
}
