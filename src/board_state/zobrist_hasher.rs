use crate::board_state::{
    board::Board, piece_type::BLACK, rng::Rng, square_index::RAW_INDEX_TO_64,
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

        for i in 0..board.w_pawns {
            let pawn_index = RAW_INDEX_TO_64[board.w_pawn_indexes[i as usize] as usize] as usize;
            let zobrist_pawn = self.zobrist_table[pawn_index][0];
            hash ^= zobrist_pawn;
        }

        for i in 0..board.b_pawns {
            let pawn_index = RAW_INDEX_TO_64[board.b_pawn_indexes[i as usize] as usize] as usize;
            let zobrist_pawn = self.zobrist_table[pawn_index][6];
            hash ^= zobrist_pawn;
        }

        for i in 0..board.w_knights {
            let knight_index =
                RAW_INDEX_TO_64[board.w_knight_indexes[i as usize] as usize] as usize;
            let zobrist_knight = self.zobrist_table[knight_index][1];
            hash ^= zobrist_knight;
        }

        for i in 0..board.b_knights {
            let knight_index =
                RAW_INDEX_TO_64[board.b_knight_indexes[i as usize] as usize] as usize;
            let zobrist_knight = self.zobrist_table[knight_index][7];
            hash ^= zobrist_knight;
        }

        for i in 0..board.w_bishops {
            let bishop_index =
                RAW_INDEX_TO_64[board.w_bishop_indexes[i as usize] as usize] as usize;
            let zobrist_bishop = self.zobrist_table[bishop_index][2];
            hash ^= zobrist_bishop;
        }

        for i in 0..board.b_bishops {
            let bishop_index =
                RAW_INDEX_TO_64[board.b_bishop_indexes[i as usize] as usize] as usize;
            let zobrist_bishop = self.zobrist_table[bishop_index][8];
            hash ^= zobrist_bishop;
        }

        for i in 0..board.w_rooks {
            let rook_index = RAW_INDEX_TO_64[board.w_rook_indexes[i as usize] as usize] as usize;
            let zobrist_rook = self.zobrist_table[rook_index][3];
            hash ^= zobrist_rook;
        }

        for i in 0..board.b_rooks {
            let rook_index = RAW_INDEX_TO_64[board.b_rook_indexes[i as usize] as usize] as usize;
            let zobrist_rook = self.zobrist_table[rook_index][9];
            hash ^= zobrist_rook;
        }

        for i in 0..board.w_queens {
            let queen_index = RAW_INDEX_TO_64[board.w_queen_indexes[i as usize] as usize] as usize;
            let zobrist_queen = self.zobrist_table[queen_index][4];
            hash ^= zobrist_queen;
        }

        for i in 0..board.b_queens {
            let queen_index = RAW_INDEX_TO_64[board.b_queen_indexes[i as usize] as usize] as usize;
            let zobrist_queen = self.zobrist_table[queen_index][10];
            hash ^= zobrist_queen;
        }

        let king_index = RAW_INDEX_TO_64[board.w_king_index as usize] as usize;
        let zobrist_white_king = self.zobrist_table[king_index][5];
        hash ^= zobrist_white_king;

        let king_index = RAW_INDEX_TO_64[board.b_king_index as usize] as usize;
        let zobrist_black_king = self.zobrist_table[king_index][11];
        hash ^= zobrist_black_king;

        hash
    }
}
