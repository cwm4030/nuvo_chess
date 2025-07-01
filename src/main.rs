mod board_state;
mod universal_chess_interface;

use std::sync::{Arc, atomic::AtomicBool};

use board_state::board::Board;
use universal_chess_interface::uci::uci_execute_command;

fn main() {
    let search_stop = Arc::new(AtomicBool::new(false));
    let mut board = Board::new();
    let mut input = String::from("position startpos");
    while uci_execute_command(&mut board, &input, &search_stop) {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
