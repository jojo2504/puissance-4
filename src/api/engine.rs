use core::{fmt};
use std::{collections::HashMap, default};
use once_cell::sync::Lazy;
use rand::random;
use ux::u42;

const EMPTY_BOARD:  u42 = u42::new(0);

const U42_ONE:      u42 = u42::new(1);
const U42_LASTBIT:  u42 = u42::new(0x20000000000);

const HEIGHT: i32 = 6;
const WIDTH: i32 = 7;

#[derive(Clone, Copy, Default)]
pub enum Color {
    #[default]
    Red,
    Yellow
}

impl Color {
    pub fn to_int(self) -> u64 {
        self as u64
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

#[derive(Default, Clone)]
pub struct Board {
    // keeping track of global board to check for valid moves
    bitboard: u42, // board is 7 col x 6 rows, same encoding as a chess board; (0, 0) is bottom left, going to right, then up
    color_bitboard: u42,
    history: Vec<(u42, Color)>, // just keep the flipped bit in history
    heights: [i32; 7]
}

impl Board {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn display_board(&self) {
        let binary_board = format!("{:042b}", self.bitboard);
        println!("{}", binary_board);
        
        let (red, yellow): (u42, u42) = match self.history.len() & 1 {
            0 => (self.color_bitboard, self.color_bitboard ^ self.bitboard),
            1 => (self.color_bitboard ^ self.bitboard, self.color_bitboard),
            _ => unreachable!(),
        };

        for row in (0..HEIGHT).rev() {
            for col in 0..WIDTH {
                let index = row * WIDTH + col;
                if u42::new(1 << index) & red != EMPTY_BOARD {
                    print!("{} ", 'R');
                }
                else if u42::new(1 << index) & yellow != EMPTY_BOARD {
                    print!("{} ", 'Y');
                }
                else {
                    print!("{} ", '0');
                }
            }
            println!();
        }
    }
}

#[derive(Default)]
pub struct Game {
    pub board: Board,
    pub turn_color: Color,
    winner: Option<Color>,
    zobrist_key: u64,
}

impl Game {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    // returns a vec of possible col to play
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

    // we are assuming the input col has already been validated
    fn push(bitboard: &mut u42, color_bitboard: &mut u42, col: i32, history: &mut Vec<(u42, Color)>, token_color: Color, zobrist_key: &mut u64, heights: &mut [i32; 7]) {
        let new_bit = u42::new(1u64 << (col + heights[col as usize] * WIDTH));
        heights[col as usize] += 1;

        *color_bitboard ^= *bitboard;
        *bitboard ^= new_bit;
        history.push((new_bit, token_color));
        *zobrist_key ^= ZOBRIST_TABLE.token_square[Zobrist::get_index((new_bit, token_color)) as usize];
    }
    
    pub fn make_push(&mut self, col: i32) {
        Self::push(&mut self.board.bitboard, &mut self.board.color_bitboard, col, &mut self.board.history, self.turn_color, &mut self.zobrist_key, &mut self.board.heights);
        self.winner = self.check_win(); 
        self.turn_color = self.turn_color.toggle();
    }

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

    pub fn check_win(&self) -> Option<Color> {
        if let Some(last_flipped_bit) = self.board.history.last() {
            // vertical;
            let mut m = self.board.color_bitboard & (self.board.color_bitboard >> (HEIGHT+1));
            if (m & (m >> (2*(HEIGHT+1)))) != EMPTY_BOARD {
                return Some(last_flipped_bit.1);
            }
            
            // horizontal 
            m = self.board.color_bitboard & (self.board.color_bitboard >> 1);
            if (m & (m >> 2)) != EMPTY_BOARD {
                return Some(last_flipped_bit.1);
            }

            // diagonal 1
            m = self.board.color_bitboard & (self.board.color_bitboard >> HEIGHT);
            if (m & (m >> (2*HEIGHT))) != EMPTY_BOARD {
                return Some(last_flipped_bit.1);
            }
            
            // diagonal 2 
            m = self.board.color_bitboard & (self.board.color_bitboard >> (HEIGHT+2));
            if (m & (m >> (2*(HEIGHT+2)))) != EMPTY_BOARD{
                return Some(last_flipped_bit.1);
            }
        }

        None
    }

    fn run(&self) {
        todo!()
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
        play.1.to_int() * 42 + raw_u64.trailing_zeros() as u64
    }
}