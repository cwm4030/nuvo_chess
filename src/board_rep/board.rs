use crate::board_rep::{
    bit_operations::{clear_bit, is_bit_set, set_bit},
    c_move::CMove,
    squares::{A1, A8, C1, C8, D1, D8, E1, E8, G1, G8, H1, H8, SQUARE_NAMES},
};

pub const WHITE: u8 = 16;
pub const BLACK: u8 = 32;
pub const COLOR_MASK: u8 = WHITE | BLACK;

pub const CASTLING_NONE: u8 = 0;
pub const CASTLING_WK: u8 = 1;
pub const CASTLING_WQ: u8 = 2;
pub const CASTLING_BK: u8 = 4;
pub const CASTLING_BQ: u8 = 8;

pub const EN_PASSANT_NONE: u8 = 64;

pub const EMPTY: u8 = 0;
pub const PAWN: u8 = 1;
pub const KNIGHT: u8 = 2;
pub const BISHOP: u8 = 3;
pub const ROOK: u8 = 4;
pub const QUEEN: u8 = 5;
pub const KING: u8 = 6;
pub const PIECE_MASK: u8 = PAWN | KNIGHT | BISHOP | ROOK | QUEEN | KING;

#[derive(Clone, Copy)]
pub struct Board {
    pub stm: u8,
    pub castling: u8,
    pub en_passant: u8,
    pub halfmove_clock: u8,
    pub fullmove_number: u16,
    pub w_pawns: u64,
    pub b_pawns: u64,
    pub w_knights: u64,
    pub b_knights: u64,
    pub w_bishops: u64,
    pub b_bishops: u64,
    pub w_rooks: u64,
    pub b_rooks: u64,
    pub w_queens: u64,
    pub b_queens: u64,
    pub w_king: u64,
    pub b_king: u64,
    pub w_occupancy: u64,
    pub b_occupancy: u64,
    pub all_occupancy: u64,
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}

impl Board {
    pub fn new() -> Self {
        Board {
            stm: WHITE,
            castling: CASTLING_NONE,
            en_passant: EN_PASSANT_NONE,
            halfmove_clock: 0,
            fullmove_number: 1,
            w_pawns: 0,
            b_pawns: 0,
            w_knights: 0,
            b_knights: 0,
            w_bishops: 0,
            b_bishops: 0,
            w_rooks: 0,
            b_rooks: 0,
            w_queens: 0,
            b_queens: 0,
            w_king: 0,
            b_king: 0,
            w_occupancy: 0,
            b_occupancy: 0,
            all_occupancy: 0,
        }
    }

    pub fn set_from_fen(&mut self, fen: &str) {
        let parts: Vec<&str> = fen.split_whitespace().filter(|s| !s.is_empty()).collect();
        let pieces = parts.first().unwrap_or(&"");
        let stm = parts.get(1).unwrap_or(&"w");
        let castling = parts.get(2).unwrap_or(&"");
        let en_passant = parts.get(3).unwrap_or(&"-");
        let halfmove_clock = parts.get(4).unwrap_or(&"0").parse::<u8>().unwrap_or(0);
        let fullmove_number = parts.get(5).unwrap_or(&"1").parse::<u16>().unwrap_or(1);

        self.reset();
        self.set_pieces(pieces);
        self.stm = if *stm == "w" { WHITE } else { BLACK };
        self.set_castling(castling);
        self.set_en_passant(en_passant);
        self.halfmove_clock = halfmove_clock;
        self.fullmove_number = fullmove_number;
    }

    pub fn print(&self, use_ascii: bool) {
        let light_background = "\x1B[48;5;39m";
        let dark_background = "\x1B[48;5;23m";
        let white = "\x1B[38;5;15m";
        let black = "\x1B[38;5;0m";
        let reset = "\x1B[0m";
        let mut background = light_background;

        println!(
            "   Side to move: {}",
            if self.stm == WHITE { "White" } else { "Black" }
        );
        println!("   Castling: {}", self.get_castling_str());
        println!(
            "   En Passant: {}",
            if self.en_passant == EN_PASSANT_NONE {
                "-"
            } else {
                SQUARE_NAMES[self.en_passant as usize]
            }
        );
        println!("   Halfmove clock: {}", self.halfmove_clock);
        println!("   Fullmove number: {}", self.fullmove_number);
        for rank in 0..8 {
            print!("   {} ", 8 - rank);
            for file in 0..8 {
                let square = (rank * 8 + file) as u8;
                let (color, piece) = self.get_color_and_piece_at(square, use_ascii);
                let foreground = if color == WHITE { white } else { black };
                print!("{foreground}{background}");
                print!(" {piece} ");
                background = if background == light_background {
                    dark_background
                } else {
                    light_background
                };
            }
            background = if background == light_background {
                dark_background
            } else {
                light_background
            };
            println!("{reset}");
        }
        println!("      a  b  c  d  e  f  g  h");
        println!();
    }

    pub fn get_piece_at(&self, square: u8) -> u8 {
        match square {
            _ if !is_bit_set(self.all_occupancy, square) => EMPTY,
            _ if is_bit_set(self.w_pawns, square) => WHITE | PAWN,
            _ if is_bit_set(self.b_pawns, square) => BLACK | PAWN,
            _ if is_bit_set(self.w_knights, square) => WHITE | KNIGHT,
            _ if is_bit_set(self.b_knights, square) => BLACK | KNIGHT,
            _ if is_bit_set(self.w_bishops, square) => WHITE | BISHOP,
            _ if is_bit_set(self.b_bishops, square) => BLACK | BISHOP,
            _ if is_bit_set(self.w_rooks, square) => WHITE | ROOK,
            _ if is_bit_set(self.b_rooks, square) => BLACK | ROOK,
            _ if is_bit_set(self.w_queens, square) => WHITE | QUEEN,
            _ if is_bit_set(self.b_queens, square) => BLACK | QUEEN,
            _ if is_bit_set(self.w_king, square) => WHITE | KING,
            _ if is_bit_set(self.b_king, square) => BLACK | KING,
            _ => EMPTY,
        }
    }

    pub fn make_move(&mut self, c_move: CMove) {
        let from_piece = self.get_piece_at(c_move.from_square);
        let to_piece = self.get_piece_at(c_move.to_square);
        if from_piece & PIECE_MASK == PAWN && c_move.to_square == self.en_passant {
            let ep_pawn_index = self.get_ep_pawn_index();
            self.set_piece_at(ep_pawn_index, EMPTY);
        } else if from_piece & PIECE_MASK == KING {
            if self.stm == WHITE {
                self.castling =
                    (self.castling | CASTLING_WK | CASTLING_WQ) ^ (CASTLING_WK | CASTLING_WQ);
            } else {
                self.castling =
                    (self.castling | CASTLING_BK | CASTLING_BQ) ^ (CASTLING_BK | CASTLING_BQ);
            }

            if c_move.from_square == E1 && c_move.to_square == G1 {
                self.set_piece_at(H1, EMPTY);
                self.set_piece_at(G1, WHITE | ROOK);
            } else if c_move.from_square == E1 && c_move.to_square == C1 {
                self.set_piece_at(A1, EMPTY);
                self.set_piece_at(D1, WHITE | ROOK);
            } else if c_move.from_square == E8 && c_move.to_square == G8 {
                self.set_piece_at(H8, EMPTY);
                self.set_piece_at(G8, BLACK | ROOK);
            } else if c_move.from_square == E8 && c_move.to_square == C8 {
                self.set_piece_at(A8, EMPTY);
                self.set_piece_at(D8, BLACK | ROOK);
            }
        }

        if c_move.from_square == A1 || c_move.to_square == A1 {
            self.castling = (self.castling | CASTLING_WQ) ^ CASTLING_WQ;
        }
        if c_move.from_square == H1 || c_move.to_square == H1 {
            self.castling = (self.castling | CASTLING_WK) ^ CASTLING_WK;
        }
        if c_move.from_square == A8 || c_move.to_square == A8 {
            self.castling = (self.castling | CASTLING_BQ) ^ CASTLING_BQ;
        }
        if c_move.from_square == H8 || c_move.to_square == H8 {
            self.castling = (self.castling | CASTLING_BK) ^ CASTLING_BK;
        }

        self.set_piece_at(c_move.from_square, EMPTY);
        self.set_piece_at(c_move.to_square, from_piece);
        if c_move.promotion != 0 {
            self.set_piece_at(c_move.to_square, EMPTY);
            self.set_piece_at(c_move.to_square, self.stm | c_move.promotion);
        }

        let rank_diff = (c_move.to_square / 8) as i8 - (c_move.from_square / 8) as i8;
        if from_piece & PIECE_MASK == PAWN && rank_diff.abs() == 2 {
            self.en_passant = c_move.to_square;
            if self.stm == WHITE {
                self.en_passant += 8;
            } else {
                self.en_passant -= 8;
            }
        } else {
            self.en_passant = EN_PASSANT_NONE;
        }

        if from_piece & PIECE_MASK == PAWN || to_piece != EMPTY {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock += 1;
        }

        self.fullmove_number += if self.stm == BLACK { 1 } else { 0 };
        self.stm ^= COLOR_MASK;
    }

    pub fn get_ep_pawn_index(&self) -> u8 {
        if self.en_passant == EN_PASSANT_NONE {
            return EN_PASSANT_NONE;
        }

        let file = (self.en_passant % 8) as i8;
        let rank = (self.en_passant / 8) as i8 + if self.stm == WHITE { -1 } else { 1 };
        (rank * 8 + file) as u8
    }

    fn get_castling_str(&self) -> String {
        let mut castling_str = String::new();
        if self.castling & CASTLING_WK != 0 {
            castling_str.push('K');
        }
        if self.castling & CASTLING_WQ != 0 {
            castling_str.push('Q');
        }
        if self.castling & CASTLING_BK != 0 {
            castling_str.push('k');
        }
        if self.castling & CASTLING_BQ != 0 {
            castling_str.push('q');
        }
        if castling_str.is_empty() {
            castling_str.push('-');
        }
        castling_str
    }

    fn get_color_and_piece_at(&self, square: u8, use_ascii: bool) -> (u8, char) {
        if square > 63 {
            return (WHITE, ' ');
        }
        match square {
            _ if self.w_pawns & (1 << square) != 0 => {
                if use_ascii {
                    (WHITE, 'P')
                } else {
                    (WHITE, '♟')
                }
            }
            _ if self.b_pawns & (1 << square) != 0 => {
                if use_ascii {
                    (BLACK, 'p')
                } else {
                    (BLACK, '♟')
                }
            }
            _ if self.w_knights & (1 << square) != 0 => {
                if use_ascii {
                    (WHITE, 'N')
                } else {
                    (WHITE, '♞')
                }
            }
            _ if self.b_knights & (1 << square) != 0 => {
                if use_ascii {
                    (BLACK, 'n')
                } else {
                    (BLACK, '♞')
                }
            }
            _ if self.w_bishops & (1 << square) != 0 => {
                if use_ascii {
                    (WHITE, 'B')
                } else {
                    (WHITE, '♝')
                }
            }
            _ if self.b_bishops & (1 << square) != 0 => {
                if use_ascii {
                    (BLACK, 'b')
                } else {
                    (BLACK, '♝')
                }
            }
            _ if self.w_rooks & (1 << square) != 0 => {
                if use_ascii {
                    (WHITE, 'R')
                } else {
                    (WHITE, '♜')
                }
            }
            _ if self.b_rooks & (1 << square) != 0 => {
                if use_ascii {
                    (BLACK, 'r')
                } else {
                    (BLACK, '♜')
                }
            }
            _ if self.w_queens & (1 << square) != 0 => {
                if use_ascii {
                    (WHITE, 'Q')
                } else {
                    (WHITE, '♛')
                }
            }
            _ if self.b_queens & (1 << square) != 0 => {
                if use_ascii {
                    (BLACK, 'q')
                } else {
                    (BLACK, '♛')
                }
            }
            _ if self.w_king & (1 << square) != 0 => {
                if use_ascii {
                    (WHITE, 'K')
                } else {
                    (WHITE, '♚')
                }
            }
            _ if self.b_king & (1 << square) != 0 => {
                if use_ascii {
                    (BLACK, 'k')
                } else {
                    (BLACK, '♚')
                }
            }
            _ => (WHITE, ' '),
        }
    }

    fn reset(&mut self) {
        self.stm = WHITE;
        self.castling = CASTLING_NONE;
        self.en_passant = EN_PASSANT_NONE;
        self.halfmove_clock = 0;
        self.fullmove_number = 1;
        self.w_pawns = 0;
        self.b_pawns = 0;
        self.w_knights = 0;
        self.b_knights = 0;
        self.w_bishops = 0;
        self.b_bishops = 0;
        self.w_rooks = 0;
        self.b_rooks = 0;
        self.w_queens = 0;
        self.b_queens = 0;
        self.w_king = 0;
        self.b_king = 0;
    }

    fn set_pieces(&mut self, pieces: &str) {
        let mut square: u8 = 0;
        for c in pieces.chars() {
            if square > 63 {
                break;
            }
            match c {
                '1'..='8' => {
                    square += c.to_digit(10).unwrap_or(1) as u8;
                }
                'P' => {
                    self.set_piece_at(square, WHITE | PAWN);
                    square += 1;
                }
                'p' => {
                    self.set_piece_at(square, BLACK | PAWN);
                    square += 1;
                }
                'N' => {
                    self.set_piece_at(square, WHITE | KNIGHT);
                    square += 1;
                }
                'n' => {
                    self.set_piece_at(square, BLACK | KNIGHT);
                    square += 1;
                }
                'B' => {
                    self.set_piece_at(square, WHITE | BISHOP);
                    square += 1;
                }
                'b' => {
                    self.set_piece_at(square, BLACK | BISHOP);
                    square += 1;
                }
                'R' => {
                    self.set_piece_at(square, WHITE | ROOK);
                    square += 1;
                }
                'r' => {
                    self.set_piece_at(square, BLACK | ROOK);
                    square += 1;
                }
                'Q' => {
                    self.set_piece_at(square, WHITE | QUEEN);
                    square += 1;
                }
                'q' => {
                    self.set_piece_at(square, BLACK | QUEEN);
                    square += 1;
                }
                'K' => {
                    self.set_piece_at(square, WHITE | KING);
                    square += 1;
                }
                'k' => {
                    self.set_piece_at(square, BLACK | KING);
                    square += 1;
                }
                _ => {}
            }
        }
    }

    fn set_piece_at(&mut self, square: u8, piece: u8) {
        if square > H1 {
            return;
        }
        match piece {
            EMPTY => {
                self.w_pawns = clear_bit(self.w_bishops, square);
                self.b_pawns = clear_bit(self.b_pawns, square);
                self.w_knights = clear_bit(self.w_knights, square);
                self.b_knights = clear_bit(self.b_knights, square);
                self.w_bishops = clear_bit(self.w_bishops, square);
                self.b_bishops = clear_bit(self.b_bishops, square);
                self.w_rooks = clear_bit(self.w_rooks, square);
                self.b_rooks = clear_bit(self.b_rooks, square);
                self.w_queens = clear_bit(self.w_queens, square);
                self.b_queens = clear_bit(self.b_queens, square);
                self.w_king = clear_bit(self.w_king, square);
                self.b_king = clear_bit(self.b_king, square);
                self.w_occupancy = clear_bit(self.w_occupancy, square);
                self.b_occupancy = clear_bit(self.b_occupancy, square);
                self.all_occupancy = clear_bit(self.all_occupancy, square);
            }
            x if x == (WHITE | PAWN) => {
                self.w_pawns = set_bit(self.w_pawns, square);
                self.w_occupancy = set_bit(self.w_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (BLACK | PAWN) => {
                self.b_pawns = set_bit(self.b_pawns, square);
                self.b_occupancy = set_bit(self.b_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (WHITE | KNIGHT) => {
                self.w_knights = set_bit(self.w_knights, square);
                self.w_occupancy = set_bit(self.w_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (BLACK | KNIGHT) => {
                self.b_knights = set_bit(self.b_knights, square);
                self.b_occupancy = set_bit(self.b_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (WHITE | BISHOP) => {
                self.w_bishops = set_bit(self.w_bishops, square);
                self.w_occupancy = set_bit(self.w_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (BLACK | BISHOP) => {
                self.b_bishops = set_bit(self.b_bishops, square);
                self.b_occupancy = set_bit(self.b_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (WHITE | ROOK) => {
                self.w_rooks = set_bit(self.w_rooks, square);
                self.w_occupancy = set_bit(self.w_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (BLACK | ROOK) => {
                self.b_rooks = set_bit(self.b_rooks, square);
                self.b_occupancy = set_bit(self.b_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (WHITE | QUEEN) => {
                self.w_queens = set_bit(self.w_queens, square);
                self.w_occupancy = set_bit(self.w_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (BLACK | QUEEN) => {
                self.b_queens = set_bit(self.b_queens, square);
                self.b_occupancy = set_bit(self.b_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (WHITE | KING) => {
                self.w_king = set_bit(self.w_king, square);
                self.w_occupancy = set_bit(self.w_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            x if x == (BLACK | KING) => {
                self.b_king = set_bit(self.b_king, square);
                self.b_occupancy = set_bit(self.b_occupancy, square);
                self.all_occupancy = set_bit(self.all_occupancy, square);
            }
            _ => {}
        }
    }

    fn set_castling(&mut self, castling: &str) {
        self.castling = CASTLING_NONE;
        for c in castling.chars() {
            match c {
                'K' => self.castling |= CASTLING_WK,
                'Q' => self.castling |= CASTLING_WQ,
                'k' => self.castling |= CASTLING_BK,
                'q' => self.castling |= CASTLING_BQ,
                _ => {}
            }
        }
    }

    fn set_en_passant(&mut self, en_passant: &str) {
        if en_passant == "-" {
            self.en_passant = EN_PASSANT_NONE;
            return;
        }

        self.en_passant = EN_PASSANT_NONE;
        let file = match en_passant.chars().next().unwrap_or('-') {
            'a' => 0,
            'b' => 1,
            'c' => 2,
            'd' => 3,
            'e' => 4,
            'f' => 5,
            'g' => 6,
            'h' => 7,
            _ => return,
        };
        let rank = match en_passant.chars().nth(1).unwrap_or('0') {
            '8' => 0,
            '7' => 1,
            '6' => 2,
            '5' => 3,
            '4' => 4,
            '3' => 5,
            '2' => 6,
            '1' => 7,
            _ => return,
        };
        self.en_passant = (rank * 8 + file) as u8;
    }
}
