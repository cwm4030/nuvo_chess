use std::time::Instant;

use crate::board_rep::{board::Board, magic_bitboards::MagicBitboards, move_gen::generate_moves};

pub fn print_perft(
    board: &Board,
    magic_bitboards: &MagicBitboards,
    mut depth: usize,
    perft_full: bool,
) {
    if depth == 0 {
        depth = 1;
    }

    let now = Instant::now();
    let mi = generate_moves(board, magic_bitboards);

    let mut total_nodes = 0;
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        if !mi.is_move_legal(board, magic_bitboards, &c_move) {
            continue;
        }
        let mut new_board = *board;
        new_board.make_move(c_move);
        let nodes = perft(&new_board, magic_bitboards, depth - 1, perft_full);
        println!("{c_move}: {nodes}");
        total_nodes += nodes;
    }
    let elapsed = now.elapsed();
    let mps = total_nodes as f64 / elapsed.as_secs_f64();
    println!();
    println!("Depth: {depth}");
    println!("Total nodes: {total_nodes}");
    println!("Seconds: {:.4}", elapsed.as_secs_f64());
    println!("Moves per second: {mps:.2}");
    println!();
}

fn perft(board: &Board, magic_bitboards: &MagicBitboards, depth: usize, perft_full: bool) -> usize {
    if depth == 0 && perft_full {
        return 1;
    }
    let mi = generate_moves(board, magic_bitboards);
    if depth == 1 && !perft_full {
        return mi.get_legal_move_count(board, magic_bitboards);
    }

    let mut total_nodes = 0;
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        if !mi.is_move_legal(board, magic_bitboards, &c_move) {
            continue;
        }
        let mut new_board = *board;
        new_board.make_move(c_move);
        total_nodes += perft(&new_board, magic_bitboards, depth - 1, perft_full);
    }
    total_nodes
}
