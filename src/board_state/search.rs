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

    for d in 1..depth + 1 {
        search_list.total_nodes = 0;
        for i in 0..search_list.count {
            let c_move = search_list.moves[i];
            board.make_move(&c_move);
            let score = -negamax(board, d - 1, i16::MIN, i16::MAX, &mut search_list);
            board.unmake_move(&c_move);

            search_list.moves[i] = c_move;
            search_list.scores[i] = score;
            search_list.nodes[i] = search_list.current_nodes;
            search_list.count = i + 1;

            search_list.total_nodes += search_list.current_nodes;
            search_list.current_nodes = 0;
        }
        search_list.sort_by_score();
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

    let mut c_move_list = CMoveList::new();
    generate_moves(board, &mut c_move_list);

    if c_move_list.count == 0 {
        search_list.current_nodes += 1;
        return if is_in_check(board) { i16::MAX } else { 0 };
    } else if board.is_possible_three_move_repetition() {
        search_list.current_nodes += 1;
        return 0;
    } else if depth == 0 {
        search_list.current_nodes += 1;
        let score = evaluate_board(board, c_move_list.count);
        return if board.stm == WHITE { score } else { -score };
    }

    let mut score = i16::MIN;
    for i in 0..c_move_list.count {
        let c_move = c_move_list.moves[i];
        board.make_move(&c_move);
        let negamax_score = -negamax(board, depth - 1, -beta, -alpha, search_list);
        score = score.max(negamax_score);
        alpha = alpha.max(score);
        board.unmake_move(&c_move);
        if alpha >= beta {
            break;
        }
    }

    score
}
