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

        let from_square = self.squares[c_move.from_index as usize];
        let to_square = self.squares[c_move.to_index as usize];
        if to_square != EMPTY_SQUARE {
            let to_piece_index: u8 = self.piece_indexes[c_move.to_index as usize];
            self.remove_from_piece_list(to_square, to_piece_index);
            self.captured_piece_history[self.history_index as usize] = to_square;
        } else if from_square & PIECE_MASK == PAWN && c_move.to_index == self.ep_index {
            let ep_pawn_index = match self.stm {
                WHITE => c_move.to_index + 16,
                _ => c_move.to_index - 16,
            };
            let ep_pawn_square = self.squares[ep_pawn_index as usize];
            self.remove_from_piece_list(ep_pawn_square, self.piece_indexes[ep_pawn_index as usize]);
            self.squares[ep_pawn_index as usize] = EMPTY_SQUARE;
            self.captured_piece_history[self.history_index as usize] = ep_pawn_square;
        } else if from_square & PIECE_MASK == KING {
            if c_move.from_index == E1 && c_move.to_index == G1 {
                self.castling_rights ^= WHITE_KING;
                self.squares[F1 as usize] = self.squares[H1 as usize];
                self.piece_indexes[F1 as usize] = self.piece_indexes[H1 as usize];
                self.squares[H1 as usize] = EMPTY_SQUARE;
                self.piece_indexes[H1 as usize] = 0;
                self.update_piece_list(
                    self.squares[F1 as usize],
                    self.piece_indexes[F1 as usize],
                    F1,
                );
            } else if c_move.from_index == E1 && c_move.to_index == C1 {
                self.castling_rights ^= WHITE_QUEEN;
                self.squares[D1 as usize] = self.squares[A1 as usize];
                self.piece_indexes[D1 as usize] = self.piece_indexes[A1 as usize];
                self.squares[A1 as usize] = EMPTY_SQUARE;
                self.piece_indexes[A1 as usize] = 0;
                self.update_piece_list(
                    self.squares[D1 as usize],
                    self.piece_indexes[D1 as usize],
                    D1,
                );
            } else if c_move.from_index == E8 && c_move.to_index == G8 {
                self.castling_rights ^= BLACK_KING;
                self.squares[F8 as usize] = self.squares[H8 as usize];
                self.piece_indexes[F8 as usize] = self.piece_indexes[H8 as usize];
                self.squares[H8 as usize] = EMPTY_SQUARE;
                self.piece_indexes[H8 as usize] = 0;
                self.update_piece_list(
                    self.squares[F8 as usize],
                    self.piece_indexes[F8 as usize],
                    F8,
                );
            } else if c_move.from_index == E8 && c_move.to_index == C8 {
                self.castling_rights ^= BLACK_QUEEN;
                self.squares[D8 as usize] = self.squares[A8 as usize];
                self.piece_indexes[D8 as usize] = self.piece_indexes[A8 as usize];
                self.squares[A8 as usize] = EMPTY_SQUARE;
                self.piece_indexes[A8 as usize] = 0;
                self.update_piece_list(
                    self.squares[D8 as usize],
                    self.piece_indexes[D8 as usize],
                    D8,
                );
            }
        }
        self.squares[c_move.to_index as usize] = self.squares[c_move.from_index as usize];
        self.piece_indexes[c_move.to_index as usize] =
            self.piece_indexes[c_move.from_index as usize];
        self.squares[c_move.from_index as usize] = EMPTY_SQUARE;
        self.piece_indexes[c_move.from_index as usize] = 0;
        self.update_piece_list(
            from_square,
            self.piece_indexes[c_move.to_index as usize],
            c_move.to_index,
        );
        if c_move.promotion_piece != 0 {
            self.squares[c_move.to_index as usize] = c_move.promotion_piece | self.stm;
            self.remove_from_piece_list(PAWN | self.stm, c_move.to_index);
            self.add_to_piece_list(c_move.promotion_piece | self.stm, c_move.to_index);
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
        self.stm = self.stm ^ OFF_BOARD_SQUARE;
        self.history_index += 1;
    }

    pub fn unmake_move(&mut self, c_move: &CMove) {
        self.history_index -= 1;
        self.ep_index = self.ep_index_history[self.history_index as usize];
        self.castling_rights = self.castling_rights_history[self.history_index as usize];
        self.halfmove = self.halfmove_history[self.history_index as usize];

        self.stm = self.stm ^ OFF_BOARD_SQUARE;
        self.fullmove = match self.stm {
            WHITE => self.fullmove,
            _ => self.fullmove - 1,
        };

        self.squares[c_move.from_index as usize] = self.squares[c_move.to_index as usize];
        self.piece_indexes[c_move.from_index as usize] =
            self.piece_indexes[c_move.to_index as usize];
        self.squares[c_move.to_index as usize] = EMPTY_SQUARE;
        self.piece_indexes[c_move.to_index as usize] = 0;
        self.update_piece_list(
            self.squares[c_move.from_index as usize],
            self.piece_indexes[c_move.from_index as usize],
            c_move.from_index,
        );

        let from_square = self.squares[c_move.from_index as usize];
        let captured_piece = self.captured_piece_history[self.history_index as usize];
        if captured_piece != EMPTY_SQUARE {
            self.squares[c_move.to_index as usize] = captured_piece;
            self.add_to_piece_list(captured_piece, c_move.to_index);
        } else if from_square & PIECE_MASK == PAWN && c_move.to_index == self.ep_index {
            let ep_pawn_index = match self.stm {
                WHITE => c_move.to_index + 16,
                _ => c_move.to_index - 16,
            };
            self.squares[ep_pawn_index as usize] = PAWN | self.stm;
            self.add_to_piece_list(PAWN | self.stm, ep_pawn_index);
        } else if from_square & PIECE_MASK == KING {
            if c_move.from_index == E1 && c_move.to_index == G1 {
                self.squares[H1 as usize] = self.squares[F1 as usize];
                self.piece_indexes[H1 as usize] = self.piece_indexes[F1 as usize];
                self.squares[F1 as usize] = EMPTY_SQUARE;
                self.piece_indexes[F1 as usize] = 0;
                self.update_piece_list(
                    self.squares[H1 as usize],
                    self.piece_indexes[H1 as usize],
                    H1,
                );
            } else if c_move.from_index == E1 && c_move.to_index == C1 {
                self.squares[A1 as usize] = self.squares[D1 as usize];
                self.piece_indexes[A1 as usize] = self.piece_indexes[D1 as usize];
                self.squares[D1 as usize] = EMPTY_SQUARE;
                self.piece_indexes[D1 as usize] = 0;
                self.update_piece_list(
                    self.squares[A1 as usize],
                    self.piece_indexes[A1 as usize],
                    A1,
                );
            } else if c_move.from_index == E8 && c_move.to_index == G8 {
                self.squares[H8 as usize] = self.squares[F8 as usize];
                self.piece_indexes[H8 as usize] = self.piece_indexes[F8 as usize];
                self.squares[F8 as usize] = EMPTY_SQUARE;
                self.piece_indexes[F8 as usize] = 0;
                self.update_piece_list(
                    self.squares[H8 as usize],
                    self.piece_indexes[H8 as usize],
                    H8,
                );
            } else if c_move.from_index == E8 && c_move.to_index == C8 {
                self.squares[A8 as usize] = self.squares[D8 as usize];
                self.piece_indexes[A8 as usize] = self.piece_indexes[D8 as usize];
                self.squares[D8 as usize] = EMPTY_SQUARE;
                self.piece_indexes[D8 as usize] = 0;
                self.update_piece_list(
                    self.squares[A8 as usize],
                    self.piece_indexes[A8 as usize],
                    A8,
                );
            }
        }

        if c_move.promotion_piece != 0 {
            self.squares[c_move.from_index as usize] = PAWN | self.stm;
            self.remove_from_piece_list(
                c_move.promotion_piece | self.stm,
                self.piece_indexes[c_move.from_index as usize],
            );
            self.add_to_piece_list(PAWN | self.stm, c_move.from_index);
        }
    }
}
