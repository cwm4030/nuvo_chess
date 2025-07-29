use std::io::Write;

use crate::board_rep::{board::Board, magic_bitboards::MagicBitboards, perft::print_perft};

pub fn uci_command(command: &str, board: &mut Board, magic_bitboards: &MagicBitboards) -> bool {
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
                let mut fen =
                    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string();
                let moves = parts.get(3..).unwrap_or(&[]).join(" ");
                if !moves.is_empty() {
                    fen = format!("{fen} moves {moves}");
                }
                board.set_from_fen(fen.as_str(), magic_bitboards);
            } else if parts.get(1).unwrap_or(&"") == &"fen" {
                let fen = parts.get(2..).unwrap_or(&[""]).join(" ");
                board.set_from_fen(fen.as_str(), magic_bitboards);
            }
            true
        }
        "quit" => false,
        "perft" => {
            let depth: usize = parts.get(1).unwrap_or(&"1").parse().unwrap_or(1);
            print_perft(board, magic_bitboards, depth, false);
            true
        }
        "perftfull" => {
            let depth: usize = parts.get(1).unwrap_or(&"1").parse().unwrap_or(1);
            print_perft(board, magic_bitboards, depth, true);
            true
        }
        "print" => {
            board.print(false);
            true
        }
        "printsimple" => {
            board.print(true);
            true
        }
        "move" => {
            let move_str = parts.get(1).unwrap_or(&"");
            board.move_from_str(magic_bitboards, move_str);
            true
        }
        "genmagics" => {
            let mut magic_bitboards = MagicBitboards::new();
            magic_bitboards.generate_magic_numbers();
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
