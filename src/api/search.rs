use std::cmp::max;
use ux::u42;

use crate::api::engine::{Board, Color, Game};

pub struct Search;

impl Search {
    fn negamax(game: &mut Game, depth: i32, mut alpha: i32, beta: i32, color: i32) -> i32 {
        if depth == 0 {
            return color * Evaluation::evaluate(&game.board);
        }

        let child_nodes = game.get_possible_moves();
        let mut best_score = i32::MIN;

        for child in child_nodes {
            game.make_push(child);
            best_score = max(best_score, Search::negamax(game, depth - 1, beta.saturating_neg(), alpha.saturating_neg(), -color).saturating_neg());
            game.unmake_push();

            alpha = max(alpha, best_score);
            if alpha >= beta {
                break;
            }
        }

        return best_score;
    }

    pub fn think(game: &mut Game) -> Option<i32> {
        let all_moves = game.get_possible_moves();
        let mut best_move: Option<i32> = None;
        let mut best_score = i32::MIN;

        for _move in all_moves {
            game.make_push(_move);
            let move_score = Search::negamax(game, 9, i32::MIN, i32::MAX, game.turn_color.to_int()).saturating_neg();
            game.unmake_push();

            if move_score > best_score {
                best_move = Some(_move);
                best_score = move_score;
            }
        }

        best_move
    }
}

pub struct Evaluation;

impl Evaluation {
    fn evaluate_window(window: u42, board: &Board) -> i32 {
        // println!("evaluating window {:042b}", window);
        // display_u42(window);
        // println!("\n");
        let (red, yellow): (u42, u42) = match board.history.len() & 1 {
            0 => (board.color_bitboard, board.color_bitboard ^ board.bitboard),
            1 => (board.color_bitboard ^ board.bitboard, board.color_bitboard),
            _ => unreachable!(),
        };

        let red_count_raw: u64 = (window & red).into();
        let red_count = red_count_raw.count_ones();
        let yellow_count_raw: u64 = (window & yellow).into();
        let yellow_count = yellow_count_raw.count_ones();

        let empty_count = 4 - red_count - yellow_count; // mask to get 0-4 range
        
        // If both colors in window, it's blocked - score 0
        if red_count > 0 && yellow_count > 0 {
            return 0;
        }

        let score = match (red_count, yellow_count, empty_count) {
            // Red (current player) patterns
            (4, 0, 0) => 100_000,      // Win
            (3, 0, 1) => 100,          // Three in a row with empty
            (2, 0, 2) => 10,           // Two in a row with 2 empty
            (1, 0, 3) => 1,            // One with 3 empty
            
            // Yellow (opponent) patterns - negative scores
            (0, 4, 0) => -100_000,     // Opponent win
            (0, 3, 1) => -100,         // Opponent threat
            (0, 2, 2) => -10,          // Opponent two in row
            (0, 1, 3) => -1,           // Opponent one piece
            
            _ => 0,
        };

        score
    }

    fn evaluation_window(board: &Board) -> i32 {
        let mut score = 0i32;
        
        // horizontals
        let mut window = u42::new(0b1111);
        for _ in 0..6 {
            for _ in 0..3 {
                score += Self::evaluate_window(window, &board);
                window <<= 1;
            }
            score += Self::evaluate_window(window, &board);
            window <<= 4;
        }
        
        // verticals
        let mut window = u42::new(0x204081);
        for _ in 0..21 {
            score += Self::evaluate_window(window, &board);
            window <<= 1;
        }
        
        // diags
        let mut ascend_diag_window = u42::new(0x1010101); 
        let mut descend_diag_window = u42::new(0x208208); 
        for _ in 0..3 {
            for _ in 0..3 {
                score += Self::evaluate_window(ascend_diag_window, &board);
                score += Self::evaluate_window(descend_diag_window, &board);
                ascend_diag_window <<= 1;
                descend_diag_window <<= 1;
            }
            score += Self::evaluate_window(ascend_diag_window, &board);
            score += Self::evaluate_window(descend_diag_window, &board);
            ascend_diag_window <<= 4;
            descend_diag_window <<= 4;
        }
        
        score
    }

    pub fn evaluate(board: &Board) -> i32 {
        let mut score = 0;
        score += Evaluation::evaluation_window(board);
        score
    }
}

struct IterativeDeepening;

impl IterativeDeepening {
}

fn display_u42(bitboard: u42) {
    for row in (0..6).rev() {
        for col in 0..7 {
            let index = row * 7 + col;
            if u42::new(1 << index) & bitboard != u42::new(0) {
                print!("{} ", '1');
            }
            else {
                print!("{} ", '0');
            }
        }
        println!("");
    }
}