const MATE: i16 = i16::MAX - 1;

use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Instant,
};

use crate::board_state::{
    board::Board,
    evaluation::evaluate_board,
    move_gen::{generate_capture_moves, generate_moves},
    piece_type::WHITE,
    search_list::SearchList,
};

pub fn alpha_beta_search(board: &mut Board, depth: usize, search_stop: &Arc<AtomicBool>) {
    let mi = generate_moves(board, true);
    let mut search_list = SearchList::new();
    search_list.set_from_c_move_list(&mi.c_move_list);
    search_list.sort_by_move_score(&mi.move_scores);

    let mut search_list_result = search_list;
    for d in 1..depth + 1 {
        let instant = Instant::now();
        search_list.nodes = 0;
        let mut alpha = i16::MIN;
        let mut beta = i16::MAX;
        let mut best_score = if board.stm == WHITE {
            i16::MIN
        } else {
            i16::MAX
        };
        for i in 0..search_list.count {
            if search_stop.load(Ordering::Relaxed) {
                println!(
                    "bestmove {}",
                    search_list_result.moves[0].get_c_move_string()
                );
                return;
            }
            let c_move = search_list.moves[i];
            board.make_move(&c_move);
            let score = alpha_beta(board, &mut search_list, search_stop, d - 1, alpha, beta);
            board.unmake_move(&c_move);

            if board.stm == WHITE {
                best_score = best_score.max(score);
                if best_score >= beta {
                    search_list.update_at_index(i, score, c_move);
                    break;
                }
                alpha = alpha.max(score);
            } else {
                best_score = best_score.min(score);
                if best_score <= alpha {
                    search_list.update_at_index(i, score, c_move);
                    break;
                }
                beta = beta.min(score);
            }
            search_list.update_at_index(i, score, c_move);
        }
        search_list.sort_by_search_score(board.stm);
        search_list_result = search_list;
        let elapsed = instant.elapsed().as_secs_f64();
        let nodes_per_second = if elapsed > 0_f64 {
            search_list.nodes as f64 / elapsed
        } else {
            f64::INFINITY
        };
        println!(
            "info depth {d} score {best_score} nodes {} time {:.2} nps {:.2}",
            search_list.nodes,
            elapsed * 1000.0,
            nodes_per_second
        );
    }
    println!(
        "bestmove {}",
        search_list_result.moves[0].get_c_move_string()
    );
}

fn alpha_beta(
    board: &mut Board,
    search_list: &mut SearchList,
    search_stop: &Arc<AtomicBool>,
    depth: usize,
    mut alpha: i16,
    mut beta: i16,
) -> i16 {
    if board.halfmove >= 50 || search_stop.load(Ordering::Relaxed) {
        search_list.nodes += 1;
        return 0;
    }

    let mut mi = generate_moves(board, true);
    if mi.c_move_list.count == 0 {
        search_list.nodes += 1;
        let mate = if board.stm == WHITE { -MATE } else { MATE };
        return if mi.check_count > 0 { mate } else { 0 };
    } else if board.is_possible_three_move_repetition() {
        search_list.nodes += 1;
        return 0;
    } else if depth == 0 {
        search_list.nodes += 1;
        return quiescence_search(board, search_list, search_stop, alpha, beta);
    }

    mi.sort_by_score();
    let mut best_score = if board.stm == WHITE {
        i16::MIN
    } else {
        i16::MAX
    };
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        board.make_move(&c_move);
        let score = alpha_beta(board, search_list, search_stop, depth - 1, alpha, beta);
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

fn quiescence_search(
    board: &mut Board,
    search_list: &mut SearchList,
    search_stop: &Arc<AtomicBool>,
    mut alpha: i16,
    mut beta: i16,
) -> i16 {
    let se = evaluate_board(board);

    if search_stop.load(Ordering::Relaxed) {
        search_list.nodes += 1;
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

    let mut mi = generate_capture_moves(board, true);
    mi.sort_by_score();
    for i in 0..mi.c_move_list.count {
        let c_move = mi.c_move_list.moves[i];
        board.make_move(&c_move);
        let score = quiescence_search(board, search_list, search_stop, alpha, beta);
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
