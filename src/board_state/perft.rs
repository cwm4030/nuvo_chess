use std::time::Instant;

use crate::board_state::{
    board::Board, c_move_list::CMoveList, move_gen::generate_moves, piece_type::OFF_BOARD_SQUARE,
};

pub fn print_perft(board: &mut Board, mut depth: usize) {
    if depth == 0 {
        depth = 1;
    }

    let now = Instant::now();
    let mut c_move_list = CMoveList::new();
    generate_moves(board, &mut c_move_list);

    let mut total_nodes = 0;
    for i in 0..c_move_list.count {
        let c_move = c_move_list.moves[i];
        board.make_move(&c_move);
        let nodes = perft(board, depth - 1);
        println!(
            "{}: {}",
            c_move.get_c_move_string(board.stm ^ OFF_BOARD_SQUARE),
            nodes
        );
        total_nodes += nodes;
        board.unmake_move(&c_move);
    }
    let elapsed = now.elapsed();
    let mps = total_nodes as f64 / elapsed.as_secs_f64();
    println!();
    println!(
        "Perft result at depth {}: {} moves in {} seconds",
        depth,
        total_nodes,
        elapsed.as_secs_f64()
    );
    println!("Moves per second: {}", mps);
    println!();
}

pub fn perft(board: &mut Board, depth: usize) -> usize {
    let mut c_move_list = CMoveList::new();
    generate_moves(board, &mut c_move_list);
    if depth == 1 {
        return c_move_list.count;
    } else if depth == 0 {
        return 1;
    }

    let mut total_nodes = 0;
    for i in 0..c_move_list.count {
        let c_move = c_move_list.moves[i];
        board.make_move(&c_move);
        total_nodes += perft(board, depth - 1);
        board.unmake_move(&c_move);
    }

    total_nodes
}
