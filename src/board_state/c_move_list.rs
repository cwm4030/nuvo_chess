use crate::board_state::c_move::CMove;

pub struct CMoveList {
    pub moves: [CMove; 256],
    pub count: usize,
}

impl CMoveList {
    pub fn new() -> Self {
        CMoveList {
            moves: [CMove {
                from_square: 0,
                to_square: 0,
                promotion_piece: 0,
            }; 256],
            count: 0,
        }
    }

    pub fn add_move(&mut self, from_square: u8, to_square: u8, promotion_piece: u8) {
        self.moves[self.count] = CMove {
            from_square,
            to_square,
            promotion_piece,
        };
        self.count += 1;
    }

    pub fn clear(&mut self) {
        self.count = 0;
    }
}
