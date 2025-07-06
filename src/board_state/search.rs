const MATE: i16 = i16::MAX - 1;

use crate::board_state::{
    board::Board,
    c_move::CMove,
    evaluation::evaluate_board,
    move_gen::{MoveInformation, generate_capture_moves, generate_moves},
    piece_type::WHITE,
    search_list::SearchList,
    search_settings::SearchSettings,
    transposition_table::TranspositionEntry,
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

        let pv = get_pv(board, search_settings, &search_list.best_move);
        let pv_string = pv
            .iter()
            .map(|m| m.get_c_move_string())
            .collect::<Vec<String>>()
            .join(" ");
        let elapsed = instant.elapsed().as_secs_f64();
        let nodes_per_second = if elapsed > 0_f64 {
            search_list.nodes as f64 / elapsed
        } else {
            f64::INFINITY
        };
        println!(
            "info depth {depth} score cp {} pv {} nodes {} time {:.2} nps {:.2}",
            search_list.best_score,
            pv_string,
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
            if sl.best_score >= beta {
                break;
            }
            if score > alpha {
                alpha = score;
                add_transposition_entry(board, search_settings, c_move);
            }
        } else {
            if score < sl.best_score {
                sl.best_score = score;
                sl.best_move = c_move;
            }
            if sl.best_score <= alpha {
                break;
            }
            if score < beta {
                beta = score;
                add_transposition_entry(board, search_settings, c_move);
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
            if best_score >= beta {
                break;
            }
            if score > alpha {
                alpha = score;
                add_transposition_entry(board, search_settings, c_move);
            }
        } else {
            best_score = best_score.min(score);
            if best_score <= alpha {
                break;
            }
            if score < beta {
                beta = score;
                add_transposition_entry(board, search_settings, c_move);
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
            if best_score >= beta {
                break;
            }
            alpha = alpha.max(score);
        } else {
            best_score = best_score.min(score);
            if best_score <= alpha {
                break;
            }
            beta = beta.min(score);
        }
    }
    best_score
}

fn add_transposition_entry(
    board: &mut Board,
    search_settings: &Arc<Mutex<SearchSettings>>,
    c_move: CMove,
) {
    search_settings
        .lock()
        .unwrap()
        .tt
        .add_entry(TranspositionEntry {
            zobrist_hash: board.zobrist_hash_history[board.history_index as usize],
            best_move: c_move,
        });
}

fn get_pv(
    board: &mut Board,
    search_settings: &Arc<Mutex<SearchSettings>>,
    best_move: &CMove,
) -> Vec<CMove> {
    let mut pv = Vec::with_capacity(64);
    pv.push(*best_move);
    let mut current_board = *board;
    current_board.make_move(best_move);
    let mut zobrist_hash = current_board
        .zobrist_hasher
        .get_zobrist_hash(&current_board);
    while let Some(entry) = search_settings.lock().unwrap().tt.get_entry(zobrist_hash) {
        let mi = generate_moves(&current_board);
        let mut found_move = false;
        for i in 0..mi.c_move_list.count {
            let c_move = mi.c_move_list.moves[i];
            if !mi.is_move_legal(&current_board, &c_move)
                || c_move.from_index != entry.best_move.from_index
                || c_move.to_index != entry.best_move.to_index
                || c_move.promotion_piece != entry.best_move.promotion_piece
            {
                continue;
            }

            found_move = true;
            pv.push(entry.best_move);
            current_board.make_move(&entry.best_move);
            zobrist_hash = current_board
                .zobrist_hasher
                .get_zobrist_hash(&current_board);
        }
        if !found_move {
            break;
        }
    }
    pv
}
