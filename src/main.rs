mod board_state;
mod universal_chess_interface;

fn main() {
    let mut board = board_state::board::Board::new();

    let mut input = String::from("position startpos");
    while universal_chess_interface::uci::uci_execute_command(&mut board, &input) {
        input.clear();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
    }
}