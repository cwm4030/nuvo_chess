use crate::board_state::board::Board;

pub fn uci_execute_command(board: &mut Board, command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let first_part = parts.first().unwrap_or(&"unknown");
    match *first_part {
        "uci" => {
            println!("id name nuvo_chess");
            println!("id author Caden Miller");
            println!("uciok");
            true
        }
        "isready" => {
            println!("readyok");
            true
        }
        "quit" => false,
        "position" => {
            if parts.len() > 2 && parts[1] == "fen" {
                let fen = parts[2..].join(" ");
                board.set_from_fen(fen.as_str());
            } else if parts.len() > 1 && parts[1] == "startpos" {
                board.set_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            } else if parts.len() > 1 && parts[1] == "print" {
                board.print_fancy_board();
            } else if parts.len() > 1 && parts[1] == "print_simple" {
                board.print_simple_board();
            }
            true
        }
        _ => true,
    }
}
