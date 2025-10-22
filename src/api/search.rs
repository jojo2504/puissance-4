use std::{cmp::max, collections::HashMap};
use ux::u42;

use crate::api::engine::{Board, File, Game};

#[derive(Default)]
enum NodeType {
    EXACT,
    LOWERBOUND,
    UPPERBOUND,
    #[default]
    None
}

#[derive(Default)]
pub struct TTEntry {
    flag: NodeType,
    depth: i32,
    value: i32
}

impl TTEntry {
    pub fn new() -> Self {
        Self {..Default::default()}
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

        let center_mask = File::D.mask();
        let center_pieces_raw: u64 = (board.color_bitboard & center_mask).into();
        let center_pieces = center_pieces_raw.count_ones() as i32;
        score += center_pieces * 3;

        score
    }
}

#[derive(Default)]
pub struct Search {
    pub depth: i32,
    pub tt: HashMap<u64, TTEntry> // zobrist_key, TTEntry
}

impl Search {
    pub fn new(depth: i32) -> Self {
        Self { 
            depth: depth,
            ..Default::default()
        }
    }

    fn negamax(&mut self, game: &mut Game, depth: i32, mut alpha: i32, beta: i32, color: i32) -> i32 {
        let alpha_orig = alpha;
        
        if let Some(tt_entry) = self.tt.get(&game.zobrist_key) && tt_entry.depth >= depth {
            match tt_entry.flag {
                NodeType::EXACT => return tt_entry.value,
                NodeType::LOWERBOUND if tt_entry.value >= beta => return tt_entry.value,
                NodeType::UPPERBOUND if tt_entry.value <= alpha => return tt_entry.value,
                _ => ()
            }
        }

        if depth == 0 || game.winner.is_some() {
            return color * Evaluation::evaluate(&game.board);
        }

        let mut child_nodes = game.get_possible_moves();
        child_nodes.sort_by_key(|&m| (m - 3).abs());
        
        let mut best_score = i32::MIN;

        for child in child_nodes {
            game.make_push(child);
            best_score = max(best_score, self.negamax(game, depth - 1, beta.saturating_neg(), alpha.saturating_neg(), -color).saturating_neg());
            game.unmake_push();

            alpha = max(alpha, best_score);
            if alpha >= beta {
                break;
            }
        }

        let mut tt_entry = TTEntry::new();
        if best_score <= alpha_orig {
            tt_entry.flag = NodeType::UPPERBOUND;
        }
        else if best_score >= beta {
            tt_entry.flag = NodeType::LOWERBOUND;
        }
        else {
            tt_entry.flag = NodeType::EXACT;
        }

        tt_entry.depth = depth;
        tt_entry.value = best_score;
        self.tt.insert(game.zobrist_key, tt_entry);

        return best_score;
    }

    pub fn think(&mut self, game: &mut Game) -> Option<i32> {
        let all_moves = game.get_possible_moves();
        let mut best_move: Option<i32> = None;
        let mut best_score = i32::MIN;

        for _move in all_moves {
            game.make_push(_move);
            let move_score = self.negamax(game, self.depth, i32::MIN, i32::MAX, game.turn_color.to_int()).saturating_neg();
            // println!("{}: {}", move_score, _move);
            game.unmake_push();

            if move_score > best_score {
                best_move = Some(_move);
                best_score = move_score;
            }
        }

        best_move
    }

    pub fn test_nets(depth1: i32, depth2: i32) {
        let mut game = Game::new();
        let mut search1 = Search::new(depth1);
        let mut search2 = Search::new(depth2);
        let mut move_history = String::new();
        loop {
            if let Some(best_move) = search1.think(&mut game) {
                game.make_push(best_move);
                move_history += &best_move.to_string();
            }

            if let Some(best_move) = search2.think(&mut game) {
                game.make_push(best_move);
                move_history += &best_move.to_string();
            }
            
            if let Some(winner) = game.winner {
                match winner {
                    super::engine::Color::Red => println!("Red won!"),
                    super::engine::Color::Yellow => println!("Yellow won!"),
                }
                game.board.display_board();
                println!("{}", move_history);
                println!("Red: Negamax(depth={})", depth1);
                println!("Yellow: Negamax(depth={})", depth2);
                println!();
                break;
            }
        }
    }
}

struct IterativeDeepening;

impl IterativeDeepening {
}