mod board_state;
mod universal_chess_interface;

use std::sync::{Arc, Mutex};

use board_state::board::Board;
use universal_chess_interface::uci::uci_execute_command;

use crate::board_state::search_settings::SearchSettings;

fn main() {
    let search_settings = Arc::new(Mutex::new(SearchSettings::new()));
    let mut board = Board::new();
    let mut input = String::from("position startpos");
    while uci_execute_command(&mut board, &search_settings, &input) {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
