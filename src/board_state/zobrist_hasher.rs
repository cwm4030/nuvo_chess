use crate::board_state::{
    board::Board,
    piece_type::{BLACK, EMPTY_SQUARE, OFF_BOARD_SQUARE, PIECE_MASK},
    rng::Rng,
    square_index::ON_BOARD_SQUARES,
};

#[derive(Copy, Clone)]
pub struct ZobristHasher {
    pub zobrist_stm: u64,
    pub zobrist_table: [[u64; 13]; 64],
    pub zobrist_ep_file: [u64; 16],
    pub zobrist_castling_rights: [u64; 16],
}

impl ZobristHasher {
    pub fn new(seed: u64) -> Self {
        let mut rng = Rng::new(seed);
        let mut zobrist_hasher = ZobristHasher {
            zobrist_stm: rng.next_u64(),
            zobrist_table: [[0; 13]; 64],
            zobrist_ep_file: [0; 16],
            zobrist_castling_rights: [0; 16],
        };

        for i in 0..64 {
            for j in 0..13 {
                zobrist_hasher.zobrist_table[i][j] = rng.next_u64();
            }
        }

        for i in 0..16 {
            zobrist_hasher.zobrist_ep_file[i] = rng.next_u64();
        }

        for i in 0..16 {
            zobrist_hasher.zobrist_castling_rights[i] = rng.next_u64();
        }

        zobrist_hasher
    }

    pub fn get_zobrist_hash(&self, board: &Board) -> u64 {
        let mut hash = 0;
        if board.stm == BLACK {
            hash ^= self.zobrist_stm;
        }

        let ep_file = board.ep_index % 16;
        if ep_file != 0 {
            hash ^= self.zobrist_ep_file[ep_file as usize];
        }

        hash ^= self.zobrist_castling_rights[board.castling_rights as usize];

        for (i, &square_index) in ON_BOARD_SQUARES.iter().enumerate() {
            let square = board.squares[square_index as usize];
            if square == EMPTY_SQUARE {
                continue;
            }
            let color_offset = match square & OFF_BOARD_SQUARE {
                BLACK => 6,
                _ => 0,
            };
            let piece_type = square & PIECE_MASK;
            let piece_index = (piece_type + color_offset) as usize;
            let zobrist_square_piece = self.zobrist_table[i][piece_index];
            hash ^= zobrist_square_piece;
        }

        hash
    }
}
