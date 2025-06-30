use std::io::Write;
use std::time::Instant;

use crate::board_state::board::Board;
use crate::board_state::evaluation::evaluate_board;
use crate::board_state::move_gen::generate_moves;
use crate::board_state::perft;
use crate::board_state::piece_type::WHITE;
use crate::board_state::search::alpha_beta_search;

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
                let depth = parts.get(2).unwrap_or(&"1").parse().unwrap_or(1) as usize;
                perft::print_perft(board, depth);
            } else if parts.len() > 1 && parts[1] == "make_move" {
                let move_str = parts.get(2).unwrap_or(&"");
                board.make_move_str(move_str);
            } else if parts.len() > 1 && parts[1] == "unmake_move" {
                board.unmake_last_move();
            } else if parts.len() > 1 && parts[1] == "evaluate" {
                let mi = generate_moves(board, false);
                let mut score = evaluate_board(board, mi.c_move_list.count);
                score = if board.stm == WHITE { score } else { -score };
                println!("Score: {:.2}", score as f32 / 100.0_f32);
                println!();
            }
            true
        }
        "go" => {
            if parts.len() > 1 && parts[1] == "depth" {
                let depth = parts.get(2).unwrap_or(&"1").parse().unwrap_or(1);
                let now = Instant::now();
                let search_list = alpha_beta_search(board, depth);
                let elapsed = now.elapsed();
                let nps = search_list.total_nodes as f64 / elapsed.as_secs_f64();
                for i in 0..search_list.count {
                    let c_move = search_list.moves[i];
                    let score = search_list.scores[i] as f32 / 100.0_f32;
                    let nodes = search_list.nodes[i];
                    println!(
                        "{}: {:.2}, {} nodes",
                        c_move.get_c_move_string(board.stm),
                        score,
                        nodes
                    );
                }
                println!(
                    "Total nodes: {total_nodes}",
                    total_nodes = search_list.total_nodes
                );
                println!(
                    "Time taken: {elapsed:.2} seconds",
                    elapsed = elapsed.as_secs_f64()
                );
                println!("Nodes per second: {nps:.2}");
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
