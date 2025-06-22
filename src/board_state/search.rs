const MATE: i16 = i16::MAX - 1;

use crate::board_state::{
    board::Board, evaluation::evaluate_board, move_gen::generate_moves, piece_type::WHITE,
    search_list::SearchList,
};

pub fn negamax_search(board: &mut Board, depth: usize) -> SearchList {
    let mi = generate_moves(board, true);
    let mut search_list = SearchList::new();
    search_list.set_from_c_move_list(&mi.c_move_list);
    search_list.sort_by_move_score();

    for d in 1..depth + 1 {
        search_list.total_nodes = 0;
        let mut alpha = i16::MIN;
        let beta = i16::MAX;
        let mut best_score = i16::MIN;
        for i in 0..search_list.count {
            let c_move = search_list.moves[i];
            board.make_move(&c_move);
            let score = -negamax(board, d - 1, -beta, -alpha, &mut search_list);
            board.unmake_move(&c_move);
            if score > best_score {
                best_score = score;
                if score > alpha {
                    alpha = score;
                }
            }
            if score >= beta {
                break;
            }

            search_list.moves[i] = c_move;
            search_list.scores[i] = score;
            search_list.nodes[i] = search_list.current_nodes;
            search_list.count = i + 1;

            search_list.total_nodes += search_list.current_nodes;
            search_list.current_nodes = 0;
        }
        search_list.sort_by_search_score();
    }

    search_list
}

fn negamax(
    board: &mut Board,
    depth: usize,
    mut alpha: i16,
    beta: i16,
    search_list: &mut SearchList,
) -> i16 {
    if board.halfmove >= 50 {
        search_list.current_nodes += 1;
        return 0;
    }

    let mut mi = generate_moves(board, true);
    if mi.c_move_list.count == 0 {
        search_list.current_nodes += 1;
        let mate = if board.stm == WHITE { MATE } else { -MATE };
        return if mi.check_count > 0 { mate } else { 0 };
    } else if board.is_possible_three_move_repetition() {
        search_list.current_nodes += 1;
        return 0;
    } else if depth == 0 {
        search_list.current_nodes += 1;
        let score = evaluate_board(board, mi.c_move_list.count);
        return if board.stm == WHITE { score } else { -score };
    }

    let mut best_score = i16::MIN;
    mi.c_move_list.sort_by_score();
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        board.make_move(&c_move);
        let score = -negamax(board, depth - 1, -beta, -alpha, search_list);
        board.unmake_move(&c_move);

        if score > best_score {
            best_score = score;
            if score > alpha {
                alpha = score;
            }
        }
        if score >= beta {
            break;
        }
    }

    best_score
}
