use crate::board_state::{c_move::CMove, c_move_list::CMoveList, piece_type::WHITE};

pub struct SearchList {
    pub moves: [CMove; 256],
    pub scores: [i16; 256],
    pub nodes: [usize; 256],
    pub current_nodes: usize,
    pub total_nodes: usize,
    pub count: usize,
}

impl SearchList {
    pub fn new() -> Self {
        SearchList {
            moves: [CMove::new(); 256],
            scores: [0; 256],
            nodes: [0; 256],
            current_nodes: 0,
            total_nodes: 0,
            count: 0,
        }
    }

    pub fn set_from_c_move_list(&mut self, c_move_list: &CMoveList) {
        self.count = c_move_list.count;
        for i in 0..self.count {
            self.moves[i] = c_move_list.moves[i];
            self.scores[i] = 0;
        }
    }

    pub fn sort_by_search_score(&mut self, stm: u8) {
        let mut indices: Vec<usize> = (0..self.count).collect();
        match stm {
            WHITE => indices.sort_by(|&a, &b| self.scores[b].cmp(&self.scores[a])),
            _ => indices.sort_by(|&a, &b| self.scores[a].cmp(&self.scores[b]))
        }

        let mut sorted_search_list = Self::new();
        for (sorted_index, &previous_index) in indices.iter().enumerate() {
            sorted_search_list.moves[sorted_index] = self.moves[previous_index];
            sorted_search_list.scores[sorted_index] = self.scores[previous_index];
            sorted_search_list.nodes[sorted_index] = self.nodes[previous_index];
        }
        sorted_search_list.current_nodes = self.current_nodes;
        sorted_search_list.total_nodes = self.total_nodes;
        sorted_search_list.count = self.count;

        *self = sorted_search_list;
    }

    pub fn sort_by_move_score(&mut self, scores: &[u16; 256]) {
        let mut indices: Vec<usize> = (0..self.count).collect();
        indices.sort_by(|&a, &b| scores[b].cmp(&scores[a]));

        let mut sorted_search_list = Self::new();
        for (sorted_index, &previous_index) in indices.iter().enumerate() {
            sorted_search_list.moves[sorted_index] = self.moves[previous_index];
            sorted_search_list.scores[sorted_index] = scores[previous_index] as i16;
            sorted_search_list.nodes[sorted_index] = self.nodes[previous_index];
        }
        sorted_search_list.current_nodes = self.current_nodes;
        sorted_search_list.total_nodes = self.total_nodes;
        sorted_search_list.count = self.count;

        *self = sorted_search_list;
    }
}
