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

    pub fn sort_by_score(&mut self, scores: &[u16; 256]) {
        let mut indices: Vec<usize> = (0..self.count).collect();
        indices.sort_by(|&a, &b| scores[b].cmp(&scores[a]));

        let mut new_moves = [CMove::new(); 256];
        for (new_index, &old_index) in indices.iter().enumerate() {
            new_moves[new_index] = self.moves[old_index];
        }
        self.moves = new_moves;
        self.count = indices.len();
    }
}
