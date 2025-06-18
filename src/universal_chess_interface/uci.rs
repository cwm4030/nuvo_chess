use std::io::Write;

use crate::board_state::board::Board;
use crate::board_state::c_move_list::CMoveList;
use crate::board_state::evaluation::evaluate_board;
use crate::board_state::move_gen::generate_moves;
use crate::board_state::perft;

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
        "position" => {
            if parts.len() > 2 && parts[1] == "fen" {
                let fen = parts[2..].join(" ");
                board.set_from_fen(fen.as_str());
            } else if parts.len() > 1 && parts[1] == "startpos" {
                let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string()
                    + " moves "
                    + &parts[2..].join(" ").to_string();
                board.set_from_fen(fen.as_str());
            } else if parts.len() > 1 && parts[1] == "print" {
                board.print_board(false);
            } else if parts.len() > 1 && parts[1] == "print_simple" {
                board.print_board(true);
            } else if parts.len() > 1 && parts[1] == "print_ascii" {
                board.print_ascii_board();
            } else if parts.len() > 1 && parts[1] == "perft" {
                let depth = parts.get(2).unwrap_or(&"1").parse().unwrap_or(1) as u64;
                perft::print_perft(board, depth);
            } else if parts.len() > 1 && parts[1] == "make_move" {
                let move_str = parts.get(2).unwrap_or(&"");
                board.make_move_str(move_str);
            } else if parts.len() > 1 && parts[1] == "unmake_move" {
                board.unmake_last_move();
            } else if parts.len() > 1 && parts[1] == "evaluate" {
                let mut c_move_list = CMoveList::new();
                generate_moves(board, &mut c_move_list);
                let score = evaluate_board(board, c_move_list.count as u16);
                println!("Score: {}", score);
                println!();
            }
            true
        }
        "clear" => {
            print!("\x1B[2J\x1B[3J\x1B[H");
            std::io::stdout().flush().unwrap();
            true
        }
        "quit" => false,
        _ => true,
    }
}
