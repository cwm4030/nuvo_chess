use crate::board_state::piece::Piece;
use crate::board_state::piece_type::{ EMPTY_SQUARE, OFF_BOARD_SQUARE, PAWN, KNIGHT, BISHOP, ROOK, QUEEN, KING, WHITE, BLACK, is_king, is_white, is_black };
use crate::board_state::square_index::{ ON_BOARD_SQUARES, ON_AND_OFF_BOARD_SQUARES, SQUARE_NAMES };
use crate::board_state::castling::{ WHITE_KING, WHITE_QUEEN, BLACK_KING, BLACK_QUEEN, get_castling_rights_string };

#[derive(Copy, Clone)]
pub struct Board {
    pub stm: u8,
    pub squares: [u8; 192],
    pub pieces: [Piece; 34],
    pub ep_index: u8,
    pub castling_rights: u8,
    pub halfmove: u8,
    pub fullmove: u16,
}

impl Board {
    pub fn new() -> Self {
        Board {
            stm: 0,
            squares: [0; 192],
            pieces: [Piece { piece_type: 0, square_index: 0 }; 34],
            ep_index: 0,
            castling_rights: 0,
            halfmove: 0,
            fullmove: 1,
        }
    }

    pub fn print_fancy_board(&self) {
        let light_background = "\x1b[48;5;39m";
        let dark_background = "\x1b[48;5;23m";
        let white = "\x1b[38;5;15m";
        let black = "\x1b[38;5;0m";
        let reset = "\x1b[0m";
        let mut foreground = white;
        let mut background = light_background;

        let stm_string = match self.stm {
            WHITE => "White",
            BLACK => "Black",
            _ => "Unknown",
        };
        let castling_rights_string = get_castling_rights_string(self.castling_rights);
        let ep_string = SQUARE_NAMES[self.ep_index as usize];
        println!("Side to move: {}", stm_string);
        println!("Castling rights: {}", castling_rights_string);
        println!("En passant square: {}", ep_string);
        println!("Halfmove clock: {}", self.halfmove);
        println!("Fullmove number: {}", self.fullmove);
        for rank in 0..8 {
            print!(" {} ", 8 - rank);
            for file in 0..8 {
                let square_index = ON_BOARD_SQUARES[rank * 8 + file] as usize;
                let piece_index = self.squares[square_index] as usize;
                let piece = self.pieces[piece_index].piece_type;
                let piece_print = match piece {
                    EMPTY_SQUARE => "   ",
                    x if x == (PAWN | BLACK) => " ♟ ",
                    x if x == (KNIGHT | BLACK) => " ♞ ",
                    x if x == (BISHOP | BLACK) => " ♝ ",
                    x if x == (ROOK | BLACK) => " ♜ ",
                    x if x == (QUEEN | BLACK) => " ♛ ",
                    x if x == (KING | BLACK) => " ♚ ",
                    x if x == (PAWN | WHITE) => " ♟ ",
                    x if x == (KNIGHT | WHITE) => " ♞ ",
                    x if x == (BISHOP | WHITE) => " ♝ ",
                    x if x == (ROOK | WHITE) => " ♜ ",
                    x if x == (QUEEN | WHITE) => " ♛ ",
                    x if x == (KING | WHITE) => " ♚ ",
                    _ => "   ",
                };
                if is_white(piece) {
                    foreground = white;
                } else if is_black(piece) {
                    foreground = black;
                }
                print!("{}{}", background, foreground);
                if background == light_background {
                    background = dark_background;
                } else {
                    background = light_background;
                }
                print!("{}", piece_print);
            }
            if background == light_background {
                background = dark_background;
            } else {
                background = light_background;
            }
            println!("{}", reset);
        }
        print!("{}", reset);
        println!("    a  b  c  d  e  f  g  h  ");
        println!();
    }

    pub fn print_simple_board(&self) {
        let stm_string = match self.stm {
            WHITE => "White",
            BLACK => "Black",
            _ => "Unknown",
        };
        let castling_rights_string = get_castling_rights_string(self.castling_rights);
        let ep_string = SQUARE_NAMES[self.ep_index as usize];
        println!("Side to move: {}", stm_string);
        println!("Castling rights: {}", castling_rights_string);
        println!("En passant square: {}", ep_string);
        println!("Halfmove clock: {}", self.halfmove);
        println!("Fullmove number: {}", self.fullmove);
        println!("   --------------------------");
        for rank in 0..8 {
            print!(" {} |", 8 - rank);
            for file in 0..8 {
                let square_index = ON_BOARD_SQUARES[rank * 8 + file] as usize;
                let piece_index = self.squares[square_index] as usize;
                let piece = self.pieces[piece_index].piece_type;
                let piece_print = match piece {
                    EMPTY_SQUARE => " . ",
                    x if x == (PAWN | BLACK) => " p ",
                    x if x == (KNIGHT | BLACK) => " n ",
                    x if x == (BISHOP | BLACK) => " b ",
                    x if x == (ROOK | BLACK) => " r ",
                    x if x == (QUEEN | BLACK) => " q ",
                    x if x == (KING | BLACK) => " k ",
                    x if x == (PAWN | WHITE) => " P ",
                    x if x == (KNIGHT | WHITE) => " N ",
                    x if x == (BISHOP | WHITE) => " B ",
                    x if x == (ROOK | WHITE) => " R ",
                    x if x == (QUEEN | WHITE) => " Q ",
                    x if x == (KING | WHITE) => " K ",
                    _ => " ? ",
                };
                print!("{}", piece_print);
            }
            println!("|");
        }
        println!("   --------------------------");
        println!("     a  b  c  d  e  f  g  h  ");
        println!();
    }

    pub fn set_from_fen(&mut self, fen: &str) {
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();
        let fen_pieces = fen_parts.get(0).unwrap_or(&"");
        let fen_stm = fen_parts.get(1).unwrap_or(&"");
        let fen_castling = fen_parts.get(2).unwrap_or(&"");
        let fen_ep = fen_parts.get(3).unwrap_or(&"");
        let fen_halfmove = fen_parts.get(4).unwrap_or(&"0");
        let fen_fullmove = fen_parts.get(5).unwrap_or(&"1");

        self.clear_squares_and_pieces();
        self.set_squares_and_pieces(fen_pieces);
        self.set_stm(fen_stm);
        self.set_castling_rights(fen_castling);
        self.set_ep(fen_ep);
        self.halfmove = fen_halfmove.parse().unwrap_or(0) as u8;
        self.fullmove = fen_fullmove.parse().unwrap_or(1) as u16;
    }

    fn clear_squares_and_pieces(&mut self) {
        for i in 0..192 {
            if ON_AND_OFF_BOARD_SQUARES[i] == 0 {
                self.squares[i] = OFF_BOARD_SQUARE;
            } else {
                self.squares[i] = EMPTY_SQUARE;
            }
        }
        for i in 0..34 {
            self.pieces[i] = Piece { piece_type: 0, square_index: 0 };
        }
    }

    fn set_squares_and_pieces(&mut self, fen_pieces: &str) {
        let mut white_pieces: usize = 3;
        let mut black_pieces: usize = 19;
        let mut on_board_square_index = 0;
        for c in fen_pieces.chars() {
            if on_board_square_index >= 64 {
                break;
            }

            if c.is_digit(10) {
                let empty_squares = c.to_digit(10).unwrap_or(0) as usize;
                on_board_square_index += empty_squares;
            } else if c == '/' {
                continue;
            } else {
                let square_index = ON_BOARD_SQUARES[on_board_square_index] as usize;
                let piece_type = match c {
                    'p' => PAWN | BLACK,
                    'n' => KNIGHT | BLACK,
                    'b' => BISHOP | BLACK,
                    'r' => ROOK | BLACK,
                    'q' => QUEEN | BLACK,
                    'k' => KING | BLACK,
                    'P' => PAWN | WHITE,
                    'N' => KNIGHT | WHITE,
                    'B' => BISHOP | WHITE,
                    'R' => ROOK | WHITE,
                    'Q' => QUEEN | WHITE,
                    'K' => KING | WHITE,
                    _ => continue,
                };
                self.set_square_and_piece(square_index, piece_type, &mut white_pieces, &mut black_pieces);
                on_board_square_index += 1;
            }
        }
    }

    fn set_square_and_piece(&mut self, square_index: usize, piece_type: u8, white_pieces: &mut usize, black_pieces: &mut usize) {
        if is_white(piece_type) {
            if is_king(piece_type) {
                self.squares[square_index] = 2;
                self.pieces[2] = Piece { piece_type, square_index: square_index as u8 };
            } else {
                self.squares[square_index] = *white_pieces as u8;
                self.pieces[*white_pieces] = Piece { piece_type, square_index: square_index as u8 };
                *white_pieces += 1;
            }
        } else if is_black(piece_type) {
            if is_king(piece_type) {
                self.squares[square_index] = 18;
                self.pieces[18] = Piece { piece_type, square_index: square_index as u8 };
            } else {
                self.squares[square_index] = *black_pieces as u8;
                self.pieces[*black_pieces] = Piece { piece_type, square_index: square_index as u8 };
                *black_pieces += 1;
            }
        }
    }

    fn set_stm(&mut self, fen_stm: &str) {
        self.stm = match fen_stm {
            "w" => WHITE,
            "b" => BLACK,
            _ => WHITE,
        };
    }

    fn set_castling_rights(&mut self, fen_castling: &str) {
        self.castling_rights = 0;
        for c in fen_castling.chars() {
            match c {
                'K' => self.castling_rights |= WHITE_KING,
                'Q' => self.castling_rights |= WHITE_QUEEN,
                'k' => self.castling_rights |= BLACK_KING,
                'q' => self.castling_rights |= BLACK_QUEEN,
                _ => continue,
            }
        }
    }

    fn set_ep(&mut self, fen_ep: &str) {
        if fen_ep == "-" {
            self.ep_index = 0;
        } else {
            let file = match fen_ep.chars().nth(0).unwrap_or('0') {
                'a' => 4,
                'b' => 5,
                'c' => 6,
                'd' => 7,
                'e' => 8,
                'f' => 9,
                'g' => 10,
                'h' => 11,
                _ => 0,
            };
            let rank = match fen_ep.chars().nth(1).unwrap_or('0') {
                '8' => 2,
                '7' => 3,
                '6' => 4,
                '5' => 5,
                '4' => 6,
                '3' => 7,
                '2' => 8,
                '1' => 9,
                _ => 0,
            };
            self.ep_index = rank * 16 + file;
        }
    }
}