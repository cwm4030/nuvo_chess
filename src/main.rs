mod board_state;
mod universal_chess_interface;

use crate::board_state::search_settings::SearchSettings;
use board_state::board::Board;
use std::sync::{Arc, Mutex};
use universal_chess_interface::uci::uci;

fn main() {
    let search_settings = Arc::new(Mutex::new(SearchSettings::new()));
    let mut board = Board::new();
    let mut input = String::from("position startpos");
    while uci(&mut board, &search_settings, &input) {
        input.clear();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
    }
}
