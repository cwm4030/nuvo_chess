use crate::board_state::{board::Board, move_gen::generate_moves};
use std::time::Instant;

pub fn print_perft(board: &mut Board, mut depth: usize) {
    if depth == 0 {
        depth = 1;
    }

    let now = Instant::now();
    let mi = generate_moves(board);

    let mut total_nodes = 0;
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        if !mi.is_move_legal(board, &c_move) {
            continue;
        }
        board.make_move(&c_move);
        let nodes = perft(board, depth - 1);
        println!("{}: {}", c_move.get_c_move_string(), nodes);
        total_nodes += nodes;
        board.unmake_move(&c_move);
    }
    let elapsed = now.elapsed();
    let mps = total_nodes as f64 / elapsed.as_secs_f64();
    println!();
    println!(
        "Perft result at depth {depth}: {total_nodes} moves in {elapsed} seconds",
        elapsed = elapsed.as_secs_f64()
    );
    println!("Moves per second: {mps}");
    println!();
}

pub fn perft(board: &mut Board, depth: usize) -> usize {
    let mi = generate_moves(board);
    if depth == 1 {
        return mi.get_num_legal_moves(board);
    } else if depth == 0 {
        return 1;
    }

    let mut total_nodes = 0;
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        if !mi.is_move_legal(board, &c_move) {
            continue;
        }
        board.make_move(&c_move);
        total_nodes += perft(board, depth - 1);
        board.unmake_move(&c_move);
    }

    total_nodes
}
