use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

use crate::board_state::board::Board;
use crate::board_state::evaluation::evaluate_board;
use crate::board_state::perft;
use crate::board_state::piece_type::WHITE;
use crate::board_state::search::search;
use crate::board_state::search_settings::SearchSettings;

pub fn uci(board: &mut Board, search_settings: &Arc<Mutex<SearchSettings>>, command: &str) -> bool {
    let parts: Vec<&str> = command.split_whitespace().collect();
    let first_part = parts.first().unwrap_or(&"unknown");
    match *first_part {
        "uci" => {
            println!("id name nuvo_chess");
            println!("id author Caden Miller");
            println!("uciok");
            true
        }
        "ucinewgame" => {
            search_settings.lock().unwrap().tt.clear();
            board.set_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
            true
        }
        "isready" => {
            println!("readyok");
            true
        }
        "print" => {
            board.print_board(false);
            true
        }
        "printsimple" => {
            board.print_board(true);
            true
        }
        "printascii" => {
            board.print_ascii_board();
            true
        }
        "move" => {
            if parts.len() > 1 {
                let move_str = parts[1];
                board.make_move_str(move_str);
            } else {
                println!("Error: No move provided");
            }
            board.print_board(false);
            true
        }
        "undomove" => {
            board.unmake_last_move();
            board.print_board(false);
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
            }
            true
        }
        "perft" => {
            if parts.len() > 1 {
                let depth = parts[1].parse().unwrap_or(1) as usize;
                perft::print_perft(board, depth);
            } else {
                println!("Error: No depth provided for perft");
            }
            true
        }
        "eval" => {
            let score = evaluate_board(board);
            println!("Eval: {:.2}", score as f32 / 100.0_f32);
            println!();
            true
        }
        "go" => {
            uci_go(board, search_settings, &parts[1..]);
            true
        }
        "stop" => {
            search_settings.lock().unwrap().stop_search = true;
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

fn uci_go(board: &mut Board, search_settings: &Arc<Mutex<SearchSettings>>, parts: &[&str]) {
    let mut depth = usize::MAX;
    let mut nodes = usize::MAX;
    let mut winc = 0.0_f64;
    let mut binc = 0.0_f64;
    let mut moves_to_go = 0;
    let mut calc_move_time = f64::MAX;
    let mut move_time = f64::MAX;
    let mut infinite = false;
    for i in (0..parts.len()).step_by(2) {
        match parts.get(i) {
            Some(&"depth") => {
                if let Some(depth_str) = parts.get(i + 1) {
                    if let Ok(d) = depth_str.parse::<usize>() {
                        depth = d;
                    }
                }
            }
            Some(&"nodes") => {
                if let Some(nodes_str) = parts.get(i + 1) {
                    if let Ok(n) = nodes_str.parse::<usize>() {
                        nodes = n;
                    }
                }
            }
            Some(&"wtime") => {
                if let Some(time_str) = parts.get(i + 1) {
                    if let Ok(time) = time_str.parse::<usize>() {
                        let wtime = time as f64 / 1000.0;
                        if board.stm == WHITE {
                            calc_move_time = wtime;
                        }
                    }
                }
            }
            Some(&"btime") => {
                if let Some(time_str) = parts.get(i + 1) {
                    if let Ok(time) = time_str.parse::<usize>() {
                        let btime = time as f64 / 1000.0;
                        if board.stm != WHITE {
                            calc_move_time = btime;
                        }
                    }
                }
            }
            Some(&"winc") => {
                if let Some(time_str) = parts.get(i + 1) {
                    if let Ok(time) = time_str.parse::<usize>() {
                        winc = time as f64 / 1000.0;
                    }
                }
            }
            Some(&"binc") => {
                if let Some(time_str) = parts.get(i + 1) {
                    if let Ok(time) = time_str.parse::<usize>() {
                        binc = time as f64 / 1000.0;
                    }
                }
            }
            Some(&"movestogo") => {
                if let Some(moves_to_go_str) = parts.get(i + 1) {
                    if let Ok(mtg) = moves_to_go_str.parse::<usize>() {
                        moves_to_go = mtg;
                    }
                }
            }
            Some(&"movetime") => {
                if let Some(move_time_str) = parts.get(i + 1) {
                    if let Ok(mt) = move_time_str.parse::<usize>() {
                        move_time = mt as f64 / 1000.0;
                    }
                }
            }
            Some(&"infinite") => {
                infinite = true;
            }
            _ => {}
        }
    }

    search_settings.lock().unwrap().stop_search = false;
    search_settings.lock().unwrap().depth = depth;
    search_settings.lock().unwrap().nodes = nodes;

    let mut time = calc_move_time;
    time += if board.stm == WHITE { winc } else { binc };
    if moves_to_go > 0 {
        time /= moves_to_go as f64;
        search_settings.lock().unwrap().time = time;
    } else {
        time /= 30.0;
        search_settings.lock().unwrap().time = time;
    }

    if move_time != f64::MAX {
        search_settings.lock().unwrap().time = move_time;
    }

    if infinite {
        *search_settings.lock().unwrap() = SearchSettings::new();
    }

    let mut board = *board;
    let search_settings = search_settings.clone();
    thread::spawn(move || {
        search_settings.lock().unwrap().start = Instant::now();
        search(&mut board, &search_settings);
    });
}
