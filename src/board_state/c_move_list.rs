use crate::board_state::c_move::CMove;

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
                score: 0,
            }; 256],
            count: 0,
        }
    }

    pub fn add_move(&mut self, from_index: u8, to_index: u8, promotion_piece: u8, score: u16) {
        self.moves[self.count] = CMove {
            from_index,
            to_index,
            promotion_piece,
            score,
        };
        self.count += 1;
    }

    pub fn sort_by_score(&mut self) {
        self.moves[0..self.count].sort_by(|a, b| b.score.cmp(&a.score));
    }
}
