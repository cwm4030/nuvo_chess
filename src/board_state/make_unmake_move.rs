use crate::board_state::{
    board::Board,
    c_move::CMove,
    castling::{BLACK_KING, BLACK_QUEEN, WHITE_KING, WHITE_QUEEN},
    piece_type::{EMPTY_SQUARE, KING, OFF_BOARD_SQUARE, PAWN, PIECE_MASK, WHITE},
    square_index::{A1, A8, C1, C8, D1, D8, E1, E8, F1, F8, G1, G8, H1, H8},
};

impl Board {
    pub fn make_move(&mut self, c_move: &CMove) {
        self.ep_index_history[self.history_index as usize] = self.ep_index;
        self.castling_rights_history[self.history_index as usize] = self.castling_rights;
        self.halfmove_history[self.history_index as usize] = self.halfmove;
        self.captured_piece_history[self.history_index as usize] = EMPTY_SQUARE;

        let from_square = self.squares[c_move.from_index as usize];
        let to_square = self.squares[c_move.to_index as usize];
        if from_square & PIECE_MASK == PAWN && c_move.to_index == self.ep_index {
            let ep_pawn_index = match self.stm {
                WHITE => c_move.to_index + 16,
                _ => c_move.to_index - 16,
            };
            let ep_pawn_square = self.squares[ep_pawn_index as usize];
            self.remove_piece(ep_pawn_index);
            self.captured_piece_history[self.history_index as usize] = ep_pawn_square;
        } else if from_square & PIECE_MASK == KING {
            if c_move.from_index == E1 && c_move.to_index == G1 {
                self.castling_rights ^= WHITE_KING;
                self.move_piece(H1, F1);
            } else if c_move.from_index == E1 && c_move.to_index == C1 {
                self.castling_rights ^= WHITE_QUEEN;
                self.move_piece(A1, D1);
            } else if c_move.from_index == E8 && c_move.to_index == G8 {
                self.castling_rights ^= BLACK_KING;
                self.move_piece(H8, F8);
            } else if c_move.from_index == E8 && c_move.to_index == C8 {
                self.castling_rights ^= BLACK_QUEEN;
                self.move_piece(A8, D8);
            }
        }
        if to_square != EMPTY_SQUARE {
            self.captured_piece_history[self.history_index as usize] = to_square;
        }
        self.move_piece(c_move.from_index, c_move.to_index);
        if c_move.promotion_piece != 0 {
            self.remove_piece(c_move.to_index);
            self.add_piece(self.stm | c_move.promotion_piece, c_move.to_index);
        }

        let move_diff = (c_move.from_index as i16 - c_move.to_index as i16).abs();
        if to_square & PIECE_MASK == PAWN && move_diff == 32 {
            self.ep_index = match self.stm {
                WHITE => c_move.to_index + 16,
                _ => c_move.to_index - 16,
            };
        }

        if from_square & PIECE_MASK == PAWN || to_square != EMPTY_SQUARE {
            self.halfmove = 0;
        } else {
            self.halfmove += 1;
        }
        self.fullmove = match self.stm {
            WHITE => self.fullmove,
            _ => self.fullmove + 1,
        };
        self.stm ^= OFF_BOARD_SQUARE;
        self.history_index += 1;
    }

    pub fn unmake_move(&mut self, c_move: &CMove) {
        self.history_index -= 1;
        self.ep_index = self.ep_index_history[self.history_index as usize];
        self.castling_rights = self.castling_rights_history[self.history_index as usize];
        self.halfmove = self.halfmove_history[self.history_index as usize];

        self.stm ^= OFF_BOARD_SQUARE;
        self.fullmove = match self.stm {
            WHITE => self.fullmove,
            _ => self.fullmove - 1,
        };

        self.move_piece(c_move.to_index, c_move.from_index);
        let from_square = self.squares[c_move.from_index as usize];
        let captured_piece = self.captured_piece_history[self.history_index as usize];
        if captured_piece != EMPTY_SQUARE {
            self.add_piece(captured_piece, c_move.to_index);
        } else if from_square & PIECE_MASK == PAWN && c_move.to_index == self.ep_index {
            let ep_pawn_index = match self.stm {
                WHITE => c_move.to_index + 16,
                _ => c_move.to_index - 16,
            };
            self.add_piece(self.stm | PAWN, ep_pawn_index);
        } else if from_square & PIECE_MASK == KING {
            if c_move.from_index == E1 && c_move.to_index == G1 {
                self.move_piece(F1, H1);
            } else if c_move.from_index == E1 && c_move.to_index == C1 {
                self.move_piece(D1, A1);
            } else if c_move.from_index == E8 && c_move.to_index == G8 {
                self.move_piece(F8, H8);
            } else if c_move.from_index == E8 && c_move.to_index == C8 {
                self.move_piece(D8, A8);
            }
        }

        if c_move.promotion_piece != 0 {
            self.remove_piece(c_move.to_index);
            self.add_piece(self.stm | PAWN, c_move.to_index);
        }
    }
}
