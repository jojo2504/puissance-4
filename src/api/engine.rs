use core::{fmt};
use std::{collections::HashMap};
use colored::Colorize;
use once_cell::sync::Lazy;
use rand::random;
use ux::u42;

use crate::{api::search::Search, gui::play::{input_difficulty, play}};

const EMPTY_BOARD:  u42 = u42::new(0);

// const U42_ONE:      u42 = u42::new(1);
const U42_LASTBIT:  u42 = u42::new(0x20000000000);

const HEIGHT: i32 = 6;
const WIDTH: i32 = 7;

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub enum Color {
    #[default]
    Red = 1,
    Yellow = -1
}

impl Color {
    pub fn to_int(self) -> i32 {
        self as i32
    }
    
    pub fn from_int(value: u64) -> Option<Self> {
        match value {
            0 => Some(Color::Red),
            1 => Some(Color::Yellow),
            _ => None,
        }
    }

    pub fn toggle(&self) -> Self {
        match self {
            Color::Red => Color::Yellow,
            Color::Yellow => Color::Red
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Yellow => write!(f, "Y")
        }
    }
}

#[derive(Clone, Copy)]
pub enum File {
    A = 0x810204081,
    B = 0x1020408102,
    C = 0x2040810204,
    D = 0x4081020408,
    E = 0x8102040810,
    F = 0x10204081020,
    G = 0x20408102040,
}

impl File {
    pub fn mask(self) -> u42 {
        u42::new(self as u64)
    }

    pub fn mask_unchecked(col: i32) -> u42 {
        match col {
            0 => File::A.mask(),
            1 => File::B.mask(),
            2 => File::C.mask(),
            3 => File::D.mask(),
            4 => File::E.mask(),
            5 => File::F.mask(),
            6 => File::G.mask(),
            _ => panic!("invalid col")
        }
    }
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            File::A => write!(f, "Base 2 (binary): {:042b}", File::A as u64),
            File::B => write!(f, "Base 2 (binary): {:042b}", File::B as u64),
            File::C => write!(f, "Base 2 (binary): {:042b}", File::C as u64),
            File::D => write!(f, "Base 2 (binary): {:042b}", File::D as u64),
            File::E => write!(f, "Base 2 (binary): {:042b}", File::E as u64),
            File::F => write!(f, "Base 2 (binary): {:042b}", File::F as u64),
            File::G => write!(f, "Base 2 (binary): {:042b}", File::G as u64),
        }
    }
}

impl TryFrom<i32> for File {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(File::A),
            1 => Ok(File::B),
            2 => Ok(File::C),
            3 => Ok(File::D),
            4 => Ok(File::E),
            5 => Ok(File::F),
            6 => Ok(File::G),
            _ => Err(()),
        }
    }
}

/// The Board struct only represent the board states and hisotry of all token bits played with their respective color.
/// 
/// Any logic related to interacting with the board is in [Game].
#[derive(Default, Clone)]
pub struct Board {
    // keeping track of global board to check for valid moves
    pub bitboard: u42, // board is 7 col x 6 rows, same encoding as a chess board; (0, 0) is bottom left, going to right, then up
    pub color_bitboard: u42,
    pub history: Vec<(u42, Color)>, // just keep the flipped bit in history
    heights: [i32; 7]
}

impl Board {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn display_board(&self) {
        // let binary_board = format!("{:042b}", self.bitboard);
        // println!("{}", binary_board);
        
        let (red, yellow): (u42, u42) = match self.history.len() & 1 {
            0 => (self.color_bitboard, self.color_bitboard ^ self.bitboard),
            1 => (self.color_bitboard ^ self.bitboard, self.color_bitboard),
            _ => unreachable!(),
        };

        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let index = row * WIDTH + col;
                if u42::new(1 << index) & red != EMPTY_BOARD {
                    print!("{} ", "R".red());
                }
                else if u42::new(1 << index) & yellow != EMPTY_BOARD {
                    print!("{} ", "Y".yellow());
                }
                else {
                    print!("{} ", '·');
                }
            }
            println!();
        }

        for i in 1..8 {
            print!("{} ", i);
        }
        println!("\n");
    }
}

/// Game allows both player to interact with the [Board], while initiliazing and keeping tracks of the zobrist key for [Search]'s transposition table.
#[derive(Default)]
pub struct Game {
    pub board: Board,
    pub turn_color: Color,
    pub winner: Option<Color>,
    pub zobrist_key: u64
}

impl Game {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }

    /// Returns a `Vec<i32>` of possible columns to play
    pub fn get_possible_moves(&self) -> Vec<i32> {
        let mut result = Vec::new();
        result.reserve_exact(WIDTH as usize);

        if self.winner.is_some() {
            return result;
        }

        let mut origin = U42_LASTBIT;
        for i in (0..WIDTH).rev() {
            if origin & self.board.bitboard == EMPTY_BOARD {
                result.push(i);
            }
            origin >>= 1;
        }
        result
    }

    /// Function helper to push a new token into the board.
    fn push(bitboard: &mut u42, color_bitboard: &mut u42, col: i32, history: &mut Vec<(u42, Color)>, token_color: Color, zobrist_key: &mut u64, heights: &mut [i32; 7]) {
        let new_bit = u42::new(1u64 << (col + heights[col as usize] * WIDTH));
        heights[col as usize] += 1;

        *color_bitboard ^= *bitboard;
        *bitboard ^= new_bit;
        history.push((new_bit, token_color));
        *zobrist_key ^= ZOBRIST_TABLE.token_square[Zobrist::get_index((new_bit, token_color)) as usize];
    }
    
    /// We are assuming the input column has already been validated and must has been validated first.
    /// Will call [`Self::push()`].
    pub fn make_push(&mut self, col: i32) {
        Self::push(&mut self.board.bitboard, &mut self.board.color_bitboard, col, &mut self.board.history, self.turn_color, &mut self.zobrist_key, &mut self.board.heights);
        self.winner = self.check_win(); 
        self.turn_color = self.turn_color.toggle();
    }

    /// Debug function to start from a game history.
    pub fn make_push_bulk(&mut self, history: &str) {
        for char in history.chars().into_iter() {
            let col: i32 = char.to_digit(10).unwrap() as i32;
            self.make_push(col);
        }
    }

    /// Unmake the last move in history.
    pub fn unmake_push(&mut self) {
        self.turn_color = self.turn_color.toggle();
        let last_play = self.board.history.pop().unwrap();
        self.zobrist_key ^= ZOBRIST_TABLE.token_square[Zobrist::get_index(last_play) as usize];

        self.board.bitboard ^= last_play.0;
        self.board.color_bitboard ^= self.board.bitboard;
        
        let col: u64 = last_play.0.into();
        self.board.heights[(col.trailing_zeros() % 7) as usize] -= 1;

        self.winner = None;
    }

    /// Check if the board has a 4-alignment for the player who just played and returns his color if true.
    pub fn check_win(&self) -> Option<Color> {
        if let Some(last_flipped_bit) = self.board.history.last() {
            let color_bitboard = self.board.color_bitboard ^ self.board.bitboard;
            let a_clear: u42 = !File::A.mask();
            let g_clear: u42 = !File::G.mask();

            // vertical go down;
            // println!("color board: {:042b}", color_bitboard);
            let m = color_bitboard & (color_bitboard >> (WIDTH));
            if (m & (m >> (2*(WIDTH)))) != EMPTY_BOARD {
                // println!("win v");
                return Some(last_flipped_bit.1);
            }
            
            // horizontal, go left and right
            let m1 = color_bitboard & (color_bitboard >> 1) & a_clear & g_clear; // To detect pairs like [_ _ X X], check if a piece has a RIGHT neighbor
            let m2 = color_bitboard & (color_bitboard << 1) & a_clear & g_clear; // To detect pairs like [X X _ _], check if a piece has a LEFT neighbor
            if (m1 & (m1 >> 2)) != EMPTY_BOARD || (m2 & (m2 << 2)) != EMPTY_BOARD {
                // println!("win h: {:042b}", m);
                return Some(last_flipped_bit.1);
            }
            
            // Diagonal ↗ (up-right) - need to prevent wraparound on both shifts
            // To detect pairs like 
            //[_ _ _ X]
            //[_ _ X _]
            let m1 = color_bitboard & (color_bitboard >> (WIDTH + 1)) & a_clear & g_clear; 
            // To detect pairs like 
            // [_ X _ _]
            // [X _ _ _]
            let m2 = color_bitboard & (color_bitboard << (WIDTH + 1)) & a_clear & g_clear; 
            if (m1 & (m1 >> 2 * (WIDTH + 1))) != EMPTY_BOARD || (m2 & (m2 << 2 * (WIDTH + 1))) != EMPTY_BOARD {
                // println!("win h: {:042b}", m);
                return Some(last_flipped_bit.1);
            }
            
            // Diagonal ↖ (up-left) - need to prevent wraparound on both shifts
            // Mask column A (and B for the second shift)
            let m1 = color_bitboard & (color_bitboard >> (WIDTH - 1)) & a_clear & g_clear;
            let m2 = color_bitboard & (color_bitboard << (WIDTH - 1)) & a_clear & g_clear;
            if (m1 & (m1 >> 2 * (WIDTH - 1))) != EMPTY_BOARD || (m2 & (m2 << 2 * (WIDTH - 1))) != EMPTY_BOARD {
                // println!("win h: {:042b}", m);
                return Some(last_flipped_bit.1);
            }
        }

        None
    }

    /// Main function to start the game.
    pub fn run(&mut self) {
        let depth = input_difficulty();
        let mut search = Search::new(depth);
        
        let mut move_history = String::new();
        self.board.display_board();
        loop {
            println!("choose a column to play (1-7): ");
            let col = play() - 1;
            if !self.get_possible_moves().contains(&col) {
                continue;
            }
            
            self.make_push(col);
            move_history += &col.to_string();
            self.board.display_board();
            
            if self.winner.is_some() {
                println!("you won !");
                break;
            }

            println!("AI is thinking...");
            if let Some(best_move) = search.think(self) {
                self.make_push(best_move);
                move_history += &best_move.to_string();
                self.board.display_board();
            }

            if self.winner.is_some() {
                println!("AI won!");
                break;
            }
        }

        println!("{}", move_history);

    }
}

#[derive(Default)]
pub struct Perft {
    pub game: Game,
    tt: HashMap<(u64, i32), u64>
}

impl Perft {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    
    pub fn run(&mut self, depth: i32) -> u64 {        
        let possible_moves = self.game.get_possible_moves();
        
        if depth == 1 {
            let count = possible_moves.len() as u64;
            return count;
        } 
        
        let mut nodes: u64 = 0;
        for possible_move in possible_moves {
            self.game.make_push(possible_move);
            nodes += self.run(depth - 1);
            self.game.unmake_push();
        }

        nodes
    }

    pub fn run_tt(&mut self, depth: i32) -> u64 {
        if let Some(&cached) = self.tt.get(&(self.game.zobrist_key, depth)) {
            return cached;
        }
        
        let possible_moves = self.game.get_possible_moves();
        
        if depth == 1 {
            let count = possible_moves.len() as u64;
            self.tt.insert((self.game.zobrist_key, depth), count);
            return count;
        } 
        
        let mut nodes: u64 = 0;
        for possible_move in possible_moves {
            self.game.make_push(possible_move);
            nodes += self.run(depth - 1);
            self.game.unmake_push();
        }

        self.tt.insert((self.game.zobrist_key, depth), nodes);
        nodes
    }

    pub fn reset(&mut self) {
        self.tt.clear();
        self.game = Default::default();
    }
}


struct Zobrist {
    token_square: [u64; 84] // 42 * 2
}

impl Default for Zobrist {
    fn default() -> Self {
        Self {
            token_square: [0u64; 84],
        }
    }
}

static ZOBRIST_TABLE: Lazy<Zobrist> = Lazy::new(|| {
    let mut z = Zobrist::default();
    for i in 0..84 {
        z.token_square[i] = random();
    }
    z
});

impl Zobrist {
    fn get_index(play: (u42, Color)) -> u64 {
        let raw_u64: u64 = play.0.into();
        let offset = match play.1.to_int() {
            1 => 0,
            -1 => 1,
            _ => unreachable!()
        };

        offset as u64 * 42 + raw_u64.trailing_zeros() as u64
    }
}