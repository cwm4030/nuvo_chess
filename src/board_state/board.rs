use crate::board_state::{
    castling::{BLACK_KING, BLACK_QUEEN, WHITE_KING, WHITE_QUEEN, get_castling_rights_string},
    piece_type::{
        BISHOP, BLACK, EMPTY_SQUARE, KING, KNIGHT, OFF_BOARD_SQUARE, PAWN, QUEEN, ROOK, WHITE,
        get_piece_string, is_black, is_white,
    },
    square_index::{ON_AND_OFF_BOARD_SQUARES, ON_BOARD_SQUARES, SQUARE_NAMES},
};

#[derive(Copy, Clone)]
pub struct Board {
    pub stm: u8,
    pub squares: [u8; 192],
    pub piece_indexes: [u8; 192],
    pub w_queens: u8,
    pub w_rooks: u8,
    pub w_bishops: u8,
    pub w_knights: u8,
    pub w_pawns: u8,
    pub w_king_index: u8,
    pub w_queen_indexes: [u8; 9],
    pub w_rook_indexes: [u8; 10],
    pub w_bishop_indexes: [u8; 10],
    pub w_knight_indexes: [u8; 10],
    pub w_pawn_indexes: [u8; 8],
    pub b_queens: u8,
    pub b_rooks: u8,
    pub b_bishops: u8,
    pub b_knights: u8,
    pub b_pawns: u8,
    pub b_king_index: u8,
    pub b_queen_indexes: [u8; 9],
    pub b_rook_indexes: [u8; 10],
    pub b_bishop_indexes: [u8; 10],
    pub b_knight_indexes: [u8; 10],
    pub b_pawn_indexes: [u8; 8],
    pub ep_index: u8,
    pub castling_rights: u8,
    pub halfmove: u8,
    pub fullmove: u16,
    pub captured_piece_history: [u8; 256],
    pub ep_index_history: [u8; 256],
    pub castling_rights_history: [u8; 256],
    pub halfmove_history: [u8; 256],
    pub history_index: u8,
}

impl Board {
    pub fn new() -> Self {
        Board {
            stm: 0,
            squares: [0; 192],
            piece_indexes: [0; 192],
            w_queens: 0,
            w_rooks: 0,
            w_bishops: 0,
            w_knights: 0,
            w_pawns: 0,
            w_king_index: 0,
            w_queen_indexes: [0; 9],
            w_rook_indexes: [0; 10],
            w_bishop_indexes: [0; 10],
            w_knight_indexes: [0; 10],
            w_pawn_indexes: [0; 8],
            b_queens: 0,
            b_rooks: 0,
            b_bishops: 0,
            b_knights: 0,
            b_pawns: 0,
            b_king_index: 0,
            b_queen_indexes: [0; 9],
            b_rook_indexes: [0; 10],
            b_bishop_indexes: [0; 10],
            b_knight_indexes: [0; 10],
            b_pawn_indexes: [0; 8],
            ep_index: 0,
            castling_rights: 0,
            halfmove: 0,
            fullmove: 1,
            captured_piece_history: [0; 256],
            ep_index_history: [0; 256],
            castling_rights_history: [0; 256],
            halfmove_history: [0; 256],
            history_index: 0,
        }
    }

    pub fn print_board(&self, use_ascii_piece: bool) {
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
                let piece_type = self.squares[square_index];
                let piece_string =
                    get_piece_string(piece_type, use_ascii_piece).replace(" . ", "   ");
                if is_white(piece_type) {
                    foreground = white;
                } else if is_black(piece_type) {
                    foreground = black;
                }
                print!("{}{}", background, foreground);
                if background == light_background {
                    background = dark_background;
                } else {
                    background = light_background;
                }
                print!("{}", piece_string);
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

    pub fn print_ascii_board(&self) {
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
                let piece_type = self.squares[square_index];
                let piece_string = get_piece_string(piece_type, true);
                print!("{}", piece_string);
            }
            println!("|");
        }
        println!("   --------------------------");
        println!("     a  b  c  d  e  f  g  h  ");
        println!();
    }

    pub fn set_from_fen(&mut self, fen: &str) {
        let fen_parts: Vec<&str> = fen.split_whitespace().collect();
        let fen_pieces = fen_parts.first().unwrap_or(&"");
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

    pub fn add_to_piece_list(&mut self, piece_type: u8, square_index: u8) {
        match piece_type {
            x if x == (BLACK | PAWN) => {
                self.b_pawn_indexes[self.b_pawns as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.b_pawns;
                self.b_pawns += 1;
            }
            x if x == (BLACK | KNIGHT) => {
                self.b_knight_indexes[self.b_knights as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.b_knights;
                self.b_knights += 1;
            }
            x if x == (BLACK | BISHOP) => {
                self.b_bishop_indexes[self.b_bishops as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.b_bishops;
                self.b_bishops += 1;
            }
            x if x == (BLACK | ROOK) => {
                self.b_rook_indexes[self.b_rooks as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.b_rooks;
                self.b_rooks += 1;
            }
            x if x == (BLACK | QUEEN) => {
                self.b_queen_indexes[self.b_queens as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.b_queens;
                self.b_queens += 1;
            }
            x if x == (BLACK | KING) => {
                self.b_king_index = square_index;
            }
            x if x == (WHITE | PAWN) => {
                self.w_pawn_indexes[self.w_pawns as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.w_pawns;
                self.w_pawns += 1;
            }
            x if x == (WHITE | KNIGHT) => {
                self.w_knight_indexes[self.w_knights as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.w_knights;
                self.w_knights += 1;
            }
            x if x == (WHITE | BISHOP) => {
                self.w_bishop_indexes[self.w_bishops as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.w_bishops;
                self.w_bishops += 1;
            }
            x if x == (WHITE | ROOK) => {
                self.w_rook_indexes[self.w_rooks as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.w_rooks;
                self.w_rooks += 1;
            }
            x if x == (WHITE | QUEEN) => {
                self.w_queen_indexes[self.w_queens as usize] = square_index;
                self.piece_indexes[square_index as usize] = self.w_queens;
                self.w_queens += 1;
            }
            x if x == (WHITE | KING) => {
                self.w_king_index = square_index;
            }
            _ => {}
        }
    }

    pub fn update_piece_list(&mut self, piece_type: u8, piece_index: u8, square_index: u8) {
        match piece_type {
            x if x == (BLACK | PAWN) => {
                self.b_pawn_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (BLACK | KNIGHT) => {
                self.b_knight_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (BLACK | BISHOP) => {
                self.b_bishop_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (BLACK | ROOK) => {
                self.b_rook_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (BLACK | QUEEN) => {
                self.b_queen_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (BLACK | KING) => {
                self.b_king_index = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (WHITE | PAWN) => {
                self.w_pawn_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (WHITE | KNIGHT) => {
                self.w_knight_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (WHITE | BISHOP) => {
                self.w_bishop_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (WHITE | ROOK) => {
                self.w_rook_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (WHITE | QUEEN) => {
                self.w_queen_indexes[piece_index as usize] = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            x if x == (WHITE | KING) => {
                self.w_king_index = square_index;
                self.piece_indexes[square_index as usize] = piece_index;
            }
            _ => {}
        }
    }

    pub fn remove_from_piece_list(&mut self, piece_type: u8, piece_index: u8) {
        match piece_type {
            x if x == (BLACK | PAWN) => {
                self.b_pawn_indexes[piece_index as usize] =
                    self.b_pawn_indexes[self.b_pawns as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.b_pawns -= 1;
            }
            x if x == (BLACK | KNIGHT) => {
                self.b_knight_indexes[piece_index as usize] =
                    self.b_knight_indexes[self.b_knights as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.b_knights -= 1;
            }
            x if x == (BLACK | BISHOP) => {
                self.b_bishop_indexes[piece_index as usize] =
                    self.b_bishop_indexes[self.b_bishops as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.b_bishops -= 1;
            }
            x if x == (BLACK | ROOK) => {
                self.b_rook_indexes[piece_index as usize] =
                    self.b_rook_indexes[self.b_rooks as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.b_rooks -= 1;
            }
            x if x == (BLACK | QUEEN) => {
                self.b_queen_indexes[piece_index as usize] =
                    self.b_queen_indexes[self.b_queens as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.b_queens -= 1;
            }
            x if x == (WHITE | PAWN) => {
                self.w_pawn_indexes[piece_index as usize] =
                    self.w_pawn_indexes[self.w_pawns as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.w_pawns -= 1;
            }
            x if x == (WHITE | KNIGHT) => {
                self.w_knight_indexes[piece_index as usize] =
                    self.w_knight_indexes[self.w_knights as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.w_knights -= 1;
            }
            x if x == (WHITE | BISHOP) => {
                self.w_bishop_indexes[piece_index as usize] =
                    self.w_bishop_indexes[self.w_bishops as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.w_bishops -= 1;
            }
            x if x == (WHITE | ROOK) => {
                self.w_rook_indexes[piece_index as usize] =
                    self.w_rook_indexes[self.w_rooks as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.w_rooks -= 1;
            }
            x if x == (WHITE | QUEEN) => {
                self.w_queen_indexes[piece_index as usize] =
                    self.w_queen_indexes[self.w_queens as usize - 1];
                self.piece_indexes[piece_index as usize] = 0;
                self.w_queens -= 1;
            }
            _ => {}
        }
    }

    fn clear_squares_and_pieces(&mut self) {
        for (i, oobs) in ON_AND_OFF_BOARD_SQUARES.iter().enumerate() {
            if *oobs == 0 {
                self.squares[i] = OFF_BOARD_SQUARE;
            } else {
                self.squares[i] = EMPTY_SQUARE;
            }
        }

        for i in 0..192 {
            self.piece_indexes[i] = 0;
        }

        self.w_king_index = 0;
        self.w_queens = 0;
        for i in 0..9 {
            self.w_queen_indexes[i] = 0;
        }

        self.w_rooks = 0;
        self.w_bishops = 0;
        self.w_knights = 0;
        for i in 0..10 {
            self.w_rook_indexes[i] = 0;
            self.w_bishop_indexes[i] = 0;
            self.w_knight_indexes[i] = 0;
        }

        self.w_pawns = 0;
        for i in 0..8 {
            self.w_pawn_indexes[i] = 0;
        }

        self.b_king_index = 0;
        self.b_queens = 0;
        for i in 0..9 {
            self.b_queen_indexes[i] = 0;
        }

        self.b_rooks = 0;
        self.b_bishops = 0;
        self.b_knights = 0;
        for i in 0..10 {
            self.b_rook_indexes[i] = 0;
            self.b_bishop_indexes[i] = 0;
            self.b_knight_indexes[i] = 0;
        }

        self.b_pawns = 0;
        for i in 0..8 {
            self.b_pawn_indexes[i] = 0;
        }
    }

    fn set_squares_and_pieces(&mut self, fen_pieces: &str) {
        let mut on_board_square_index = 0;
        for c in fen_pieces.chars() {
            if on_board_square_index >= 64 {
                break;
            }

            if c.is_ascii_digit() {
                let empty_squares = c.to_digit(10).unwrap_or(0) as usize;
                on_board_square_index += empty_squares;
            } else if c == '/' {
                continue;
            } else {
                let square_index = ON_BOARD_SQUARES[on_board_square_index] as usize;
                match c {
                    'p' => {
                        self.squares[square_index] = BLACK | PAWN;
                        self.add_to_piece_list(BLACK | PAWN, square_index as u8);
                    }
                    'n' => {
                        self.squares[square_index] = BLACK | KNIGHT;
                        self.add_to_piece_list(BLACK | KNIGHT, square_index as u8);
                    }
                    'b' => {
                        self.squares[square_index] = BLACK | BISHOP;
                        self.add_to_piece_list(BLACK | BISHOP, square_index as u8);
                    }
                    'r' => {
                        self.squares[square_index] = BLACK | ROOK;
                        self.add_to_piece_list(BLACK | ROOK, square_index as u8);
                    }
                    'q' => {
                        self.squares[square_index] = BLACK | QUEEN;
                        self.add_to_piece_list(BLACK | QUEEN, square_index as u8);
                    }
                    'k' => {
                        self.squares[square_index] = BLACK | KING;
                        self.add_to_piece_list(BLACK | KING, square_index as u8);
                        self.b_king_index = square_index as u8;
                    }
                    'P' => {
                        self.squares[square_index] = WHITE | PAWN;
                        self.add_to_piece_list(WHITE | PAWN, square_index as u8);
                    }
                    'N' => {
                        self.squares[square_index] = WHITE | KNIGHT;
                        self.add_to_piece_list(WHITE | KNIGHT, square_index as u8);
                    }
                    'B' => {
                        self.squares[square_index] = WHITE | BISHOP;
                        self.add_to_piece_list(WHITE | BISHOP, square_index as u8);
                    }
                    'R' => {
                        self.squares[square_index] = WHITE | ROOK;
                        self.add_to_piece_list(WHITE | ROOK, square_index as u8);
                    }
                    'Q' => {
                        self.squares[square_index] = WHITE | QUEEN;
                        self.add_to_piece_list(WHITE | QUEEN, square_index as u8);
                    }
                    'K' => {
                        self.squares[square_index] = WHITE | KING;
                        self.add_to_piece_list(WHITE | KING, square_index as u8);
                        self.w_king_index = square_index as u8;
                    }
                    _ => continue,
                };
                on_board_square_index += 1;
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
        if file == 0 || rank == 0 {
            self.ep_index = 0;
        } else {
            self.ep_index = rank * 16 + file;
        }
    }
}
