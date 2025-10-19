use std::cmp::max;
use crate::api::engine::{Board, Color, Game};

struct Search;

impl Search {
    fn negamax(game: &mut Game, depth: i32, mut alpha: i64, beta: i64, color: i32) -> i64 {
        if depth == 0 {
            return color as i64 * Evaluation::evaluate(game.board.clone());
        }

        let child_nodes = game.get_possible_moves();
        let mut value = i64::MIN;

        for child in child_nodes {
            game.make_push(child);
            value = max(value, -Search::negamax(game, depth - 1, -beta, -alpha, -color));
            game.unmake_push();

            alpha = max(alpha, value);
            if alpha >= beta {
                break;
            }
        }

        return value;
    }
}

struct Evaluation;

impl Evaluation {
    fn evaluate(board: Board) -> i64 {
        todo!()
    }
}

struct IterativeDeepening;

impl IterativeDeepening {
}