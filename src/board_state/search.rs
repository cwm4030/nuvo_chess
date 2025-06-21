use crate::board_state::{
    board::Board,
    c_move_list::CMoveList,
    evaluation::evaluate_board,
    move_gen::{generate_moves, is_in_check},
    piece_type::WHITE,
    search_list::SearchList,
};

pub fn negamax_search(board: &mut Board, depth: usize) -> SearchList {
    let mut c_move_list = CMoveList::new();
    generate_moves(board, &mut c_move_list);
    let mut search_list = SearchList::new();
    search_list.set_from_c_move_list(&c_move_list);

    let mut alpha = if board.stm == WHITE {
        -10000.0
    } else {
        10000.0
    };
    let beta = if board.stm == WHITE {
        10000.0
    } else {
        -10000.0
    };
    for d in 1..depth + 1 {
        search_list.total_nodes = 0;
        for i in 0..search_list.count {
            let c_move = search_list.moves[i];
            board.make_move(&c_move);
            let (mut score, node_count) = negamax(board, d - 1, -beta, -alpha, 0);
            score = -score;
            board.unmake_move(&c_move);
            if score > alpha {
                alpha = score;
            }
            search_list.moves[i] = c_move;
            search_list.scores[i] = score;
            search_list.nodes[i] = node_count;
            search_list.total_nodes += node_count;
            search_list.count = i + 1;
        }
        search_list.sort_by_score();
    }

    search_list
}

fn negamax(
    board: &mut Board,
    depth: usize,
    mut alpha: f32,
    beta: f32,
    nodes: usize,
) -> (f32, usize) {
    if board.halfmove >= 50 {
        return (0.0, nodes + 1);
    }

    let mut c_move_list = CMoveList::new();
    generate_moves(board, &mut c_move_list);

    if c_move_list.count == 0 {
        return if is_in_check(board) {
            (10000.0, nodes + 1)
        } else {
            (0.0, nodes + 1)
        };
    } else if board.is_possible_three_move_repetition() {
        return (0.0, nodes + 1);
    } else if depth == 0 {
        return (evaluate_board(board, c_move_list.count), nodes + 1);
    }

    let mut total_nodes = nodes;
    let mut score = -10000.0;
    for i in 0..c_move_list.count {
        let c_move = c_move_list.moves[i];
        board.make_move(&c_move);
        let (negamax_score, negamax_nodes) = negamax(board, depth - 1, -beta, -alpha, total_nodes);
        score = -negamax_score;
        total_nodes = negamax_nodes + 1;
        board.unmake_move(&c_move);
        if score >= beta {
            return (beta, total_nodes);
        }
        if score > alpha {
            alpha = score;
        }
    }

    (score, total_nodes)
}
