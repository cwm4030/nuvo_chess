const MATE: i16 = i16::MAX - 1;

use crate::board_state::{
    board::Board,
    c_move::CMove,
    evaluation::evaluate_board,
    move_gen::{MoveInformation, generate_capture_moves, generate_moves},
    piece_type::WHITE,
    search_list::SearchList,
    search_settings::SearchSettings,
};
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

pub fn search(board: &mut Board, search_settings: &Arc<Mutex<SearchSettings>>) {
    let mut mi = generate_moves(board);
    mi.score_moves(board);

    let mut search_list = SearchList::new();
    search_list.set_from_c_move_list(&mi.c_move_list);
    search_list.sort_by_move_score(&mi.move_scores);

    let mut depth: usize = 1;
    loop {
        if search_settings
            .lock()
            .unwrap()
            .should_stop_search(depth, search_list.nodes)
        {
            break;
        }

        let instant: Instant = Instant::now();
        search_list.completed = false;
        search_list = search_at_depth(
            board,
            &search_list,
            search_settings,
            &mi,
            depth,
            i16::MIN,
            i16::MAX,
        );
        if !search_list.completed {
            break;
        }

        let elapsed = instant.elapsed().as_secs_f64();
        let nodes_per_second = if elapsed > 0_f64 {
            search_list.nodes as f64 / elapsed
        } else {
            f64::INFINITY
        };
        println!(
            "info depth {depth} score cp {} pv {} nodes {} time {:.2} nps {:.2}",
            search_list.best_score,
            search_list.best_move.get_c_move_string(),
            search_list.nodes,
            elapsed * 1000.0,
            nodes_per_second
        );
        depth += 1;
    }
    println!("bestmove {}", search_list.best_move.get_c_move_string());
}

fn search_at_depth(
    board: &mut Board,
    search_list: &SearchList,
    search_settings: &Arc<Mutex<SearchSettings>>,
    mi: &MoveInformation,
    depth: usize,
    mut alpha: i16,
    mut beta: i16,
) -> SearchList {
    let mut sl = *search_list;
    sl.best_move = CMove::new();
    sl.best_score = if board.stm == WHITE {
        i16::MIN
    } else {
        i16::MAX
    };
    for i in 0..sl.count {
        if search_settings
            .lock()
            .unwrap()
            .should_stop_search(depth, search_list.nodes)
        {
            return *search_list;
        }

        let c_move = search_list.moves[i];
        if !mi.is_move_legal(board, &c_move) {
            continue;
        }

        board.make_move(&c_move);
        board.zobrist_hash_history[board.history_index as usize] =
            board.zobrist_hasher.get_zobrist_hash(board);
        let score = alpha_beta(board, &mut sl, search_settings, depth - 1, alpha, beta);
        board.unmake_move(&c_move);

        if board.stm == WHITE {
            if score > sl.best_score {
                sl.best_score = score;
                sl.best_move = c_move;
            }
            alpha = alpha.max(score);
            if sl.best_score >= beta {
                break;
            }
        } else {
            if score < sl.best_score {
                sl.best_score = score;
                sl.best_move = c_move;
            }
            beta = beta.min(score);
            if sl.best_score <= alpha {
                break;
            }
        }
        sl.update_at_index(i, score, c_move);
    }
    sl.sort_by_search_score(board.stm);
    sl.completed = true;
    sl
}

fn alpha_beta(
    board: &mut Board,
    search_list: &mut SearchList,
    search_settings: &Arc<Mutex<SearchSettings>>,
    depth: usize,
    mut alpha: i16,
    mut beta: i16,
) -> i16 {
    if board.halfmove >= 50 {
        search_list.nodes += 1;
        return 0;
    } else if search_list.nodes & 2048 == 0
        && search_settings
            .lock()
            .unwrap()
            .should_stop_search(depth, search_list.nodes)
    {
        return 0;
    }

    let mut mi = generate_moves(board);
    if mi.get_num_legal_moves(board) == 0 {
        search_list.nodes += 1;
        let mate = if board.stm == WHITE { -MATE } else { MATE };
        return if mi.check_count > 0 { mate } else { 0 };
    } else if board.is_possible_three_move_repetition() {
        search_list.nodes += 1;
        return 0;
    } else if depth == 0 {
        search_list.nodes += 1;
        return quiescence_search(board, search_list, search_settings, alpha, beta);
    }

    mi.score_moves(board);
    mi.sort_by_score();
    let mut best_score = if board.stm == WHITE {
        i16::MIN
    } else {
        i16::MAX
    };
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        if !mi.is_move_legal(board, &c_move) {
            continue;
        }
        board.make_move(&c_move);
        board.zobrist_hash_history[board.history_index as usize] =
            board.zobrist_hasher.get_zobrist_hash(board);
        let score = alpha_beta(board, search_list, search_settings, depth - 1, alpha, beta);
        board.unmake_move(&c_move);

        if board.stm == WHITE {
            best_score = best_score.max(score);
            alpha = alpha.max(score);
            if best_score >= beta {
                break;
            }
        } else {
            best_score = best_score.min(score);
            beta = beta.min(score);
            if best_score <= alpha {
                break;
            }
        }
    }
    best_score
}

fn quiescence_search(
    board: &mut Board,
    search_list: &mut SearchList,
    search_settings: &Arc<Mutex<SearchSettings>>,
    mut alpha: i16,
    mut beta: i16,
) -> i16 {
    let se = evaluate_board(board);
    if search_settings
        .lock()
        .unwrap()
        .should_stop_search(0, search_list.nodes)
    {
        return se;
    }
    let mut best_score = se;
    if board.stm == WHITE {
        if best_score >= beta {
            search_list.nodes += 1;
            return best_score;
        }
    } else if best_score <= alpha {
        search_list.nodes += 1;
        return best_score;
    }

    let mut mi = generate_capture_moves(board);
    mi.score_moves(board);
    mi.sort_by_score();
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        if !mi.is_move_legal(board, &c_move) {
            continue;
        }
        board.make_move(&c_move);
        board.zobrist_hash_history[board.history_index as usize] =
            board.zobrist_hasher.get_zobrist_hash(board);
        let score = quiescence_search(board, search_list, search_settings, alpha, beta);
        board.unmake_move(&c_move);

        if board.stm == WHITE {
            best_score = best_score.max(score);
            alpha = alpha.max(score);
            if best_score >= beta {
                break;
            }
        } else {
            best_score = best_score.min(score);
            beta = beta.min(score);
            if best_score <= alpha {
                break;
            }
        }
    }
    best_score
}
