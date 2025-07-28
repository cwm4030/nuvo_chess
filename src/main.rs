use crate::board_rep::{board::Board, magic_bitboards::MagicBitboards};

pub mod board_rep;
pub mod uci;

fn main() {
    let mut board = Board::new();
    let magic_bitboards = MagicBitboards::new();
    board.set_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");

    let mut input = String::new();
    loop {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
        if !uci::uci_command(input.as_str(), &mut board, &magic_bitboards) {
            break;
        }
    }
}
