use crate::board_rep::c_move::CMove;

pub struct CMoveList {
    pub moves: [CMove; 256],
    pub count: usize,
}

impl Default for CMoveList {
    fn default() -> Self {
        CMoveList::new()
    }
}

impl CMoveList {
    pub fn new() -> Self {
        CMoveList {
            moves: [CMove::default(); 256],
            count: 0,
        }
    }

    pub fn add_move(&mut self, from_square: u8, to_square: u8, promotion: u8) {
        let c_move = CMove {
            from_square,
            to_square,
            promotion,
        };
        self.moves[self.count] = c_move;
        self.count += 1;
    }
}
