use std::io::Write;

use crate::board_rep::board::Board;

pub fn uci_command(command: &str, board: &mut Board) -> bool {
    let parts: Vec<&str> = command
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect();
    match *parts.first().unwrap_or(&"") {
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
        "position" => {
            if parts.get(1).unwrap_or(&"") == &"startpos" {
                board.set_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            } else if parts.get(1).unwrap_or(&"") == &"fen" {
                let fen = parts.get(2..).unwrap_or(&[""]).join(" ");
                board.set_from_fen(fen.as_str());
            }
            true
        }
        "quit" => false,
        "print" => {
            board.print(false);
            true
        }
        "printsimple" => {
            board.print(true);
            true
        }
        "clear" => {
            print!("\x1B[2J\x1B[3J\x1B[H");
            std::io::stdout().flush().unwrap();
            true
        }
        _ => true,
    }
}
