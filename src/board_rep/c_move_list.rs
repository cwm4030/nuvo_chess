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

    pub fn add_move(&mut self, from: u8, to: u8, promotion: u8) {
        let c_move = CMove {
            from,
            to,
            promotion,
        };
        self.moves[self.count] = c_move;
        self.count += 1;
    }
}
