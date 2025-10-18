use core::{fmt};
use std::{collections::HashMap};
use once_cell::sync::Lazy;
use rand::random;
use ux::u42;

const EMPTY_BOARD:  u42 = u42::new(0);

const TOPRIGHT:     u42 = u42::new(0x20408102040 | 0x3F800000000);
const BOTTOMLEFT:   u42 = u42::new(0x810204081 | 0x7F);
const TOPLEFT:      u42 = u42::new(0x810204081 | 0x3F800000000);
const BOTTOMRIGHT:  u42 = u42::new(0x20408102040 | 0x7F);

const U42_ONE:      u42 = u42::new(1);
const U42_LASTBIT:  u42 = u42::new(0x20000000000);

#[derive(Clone, PartialEq, Eq, Copy, Default)]
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

#[derive(Default)]
pub struct Board {
    // keeping track of global board to check for valid moves
    bitboard: u42, // board is 7 col x 6 rows, same encoding as a chess board; (0, 0) is bottom left, going to right, then up
    yellow: u42,
    red: u42,
    history: Vec<(u42, Color)> // just keep the flipped bit in history
}

fn get_msb(bitboard: u42) -> Option<u42> {
    if bitboard == EMPTY_BOARD {
        return None;
    }

    let raw: u64 = bitboard.into();
    return Some(U42_ONE << (63 - raw.leading_zeros()));
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
        
        for row in (0..6).rev() {
            for col in 0..7 {
                let index = row * 7 + col;
                if u42::new(1 << index) & self.red != EMPTY_BOARD {
                    print!("{} ", 'R');
                }
                else if u42::new(1 << index) & self.yellow != EMPTY_BOARD {
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

fn count_direction(origin: u42, color: &u42, shift: i32, bound: u42) -> i32 {
    let mut count = 0;
    let mut mask = origin;

    loop {
        if mask & bound != EMPTY_BOARD {
            break;
        }
        
        if shift > 0 {
            mask <<= shift;
        }
        else {
            mask >>= -shift;
        }
        
        if mask & *color != EMPTY_BOARD {
            count += 1;
        }
    }
    count
}

#[derive(Default)]
pub struct Game {
    board: Board,
    turn_color: Color,
    winner: Option<Color>,
    zobrist_key: u64
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            ..Default::default()
        }
    }

    // returns a vec of possible col to play
    pub fn get_possible_moves(&self) -> Vec<i32> {
        let mut result = Vec::new();
        result.reserve_exact(7);

        if self.winner.is_some() {
            return result;
        }

        let mut origin = U42_LASTBIT;
        for i in (0..7).rev() {
            if origin & self.board.bitboard == EMPTY_BOARD {
                result.push(i);
            }
            origin >>= 1;
        }
        result
    }

    // we are assuming the input col has already been validated
    fn push(bitboard: &mut u42, color_bitboard: &mut u42, col: i32, history: &mut Vec<(u42, Color)>, token_color: Color, zobrist_key: &mut u64) {
        if let Ok(file) = File::try_from(col) {
            if let Some(place_index) = get_msb(*bitboard & file.mask()) {
                let future_place_index = place_index << 7;
                *color_bitboard ^= future_place_index;
                *bitboard ^= future_place_index;
                history.push((future_place_index, token_color));
                *zobrist_key ^= ZOBRIST_TABLE.token_square[Zobrist::get_index((future_place_index, token_color)) as usize];
            }
            else {
                let default_index= u42::new(1 << col);
                *color_bitboard ^= default_index;
                *bitboard ^= default_index;
                history.push((default_index, token_color));
                *zobrist_key ^= ZOBRIST_TABLE.token_square[Zobrist::get_index((default_index, token_color)) as usize];
            }
        }
    }
    
    pub fn make_push(&mut self, col: i32) {
        match self.turn_color {
            Color::Red => {
                Self::push(&mut self.board.bitboard, &mut self.board.red, col, &mut self.board.history, self.turn_color, &mut self.zobrist_key);
            },
            Color::Yellow => {
                Self::push(&mut self.board.bitboard, &mut self.board.yellow, col, &mut self.board.history, self.turn_color, &mut self.zobrist_key);
            }
        }
        self.winner = self.check_win(); 
        self.turn_color = self.turn_color.toggle();
    }

    pub fn unmake_push(&mut self) {
        self.turn_color = self.turn_color.toggle();
        let last_play = self.board.history.pop().unwrap();
        self.zobrist_key ^= ZOBRIST_TABLE.token_square[Zobrist::get_index(last_play) as usize];

        self.board.bitboard ^= last_play.0;
        match last_play.1 {
            Color::Red => {
                self.board.red ^= last_play.0;
            },
            Color::Yellow => {
                self.board.yellow ^= last_play.0;
            }
        }

        self.winner = None;
    }

    pub fn check_win(&self) -> Option<Color> {
        if let Some(last_flipped_bit) = self.board.history.last() {
            let raw_u64: u64 = last_flipped_bit.0.into();
            let index = raw_u64.trailing_zeros();
            let col = index % 7;
            let color;
            match last_flipped_bit.1 {
                Color::Red => color = &self.board.red,
                Color::Yellow => color = &self.board.yellow,
            }

            // horizontal check _
            let mut origin = last_flipped_bit.0 >> col;
            let mut counter = 0;
            for _ in 0..7 {
                if origin & *color != EMPTY_BOARD {
                    counter += 1;
                }
                else {
                    counter = 0;
                }
                origin <<= 1;
                if counter >= 4 {
                    return Some(last_flipped_bit.1);
                }
            }

            // vertical check |
            origin = u42::new(1 << col);
            for _ in 0..6 {
                if origin & *color != EMPTY_BOARD {
                    counter += 1;
                }
                else {
                    counter = 0;
                }
                origin <<= 7;
                if counter >= 4 {
                    return Some(last_flipped_bit.1);
                }
            }

            // anti diagonal check /
            let mut diag_counter = 1;
            diag_counter += count_direction(last_flipped_bit.0, color, 8, TOPRIGHT);  // if n>0  << n, top direction
            diag_counter += count_direction(last_flipped_bit.0, color, -8, BOTTOMLEFT); // if n<0  >> -n, 
            if diag_counter >= 4 {
                return Some(last_flipped_bit.1);
            }
            
            // main diagonal check \ >> 8 
            diag_counter = 1; 
            diag_counter += count_direction(last_flipped_bit.0, color, 6, TOPLEFT);  // if n>0  << n, top direction
            diag_counter += count_direction(last_flipped_bit.0, color, -6, BOTTOMRIGHT); // if n<0  >> -n 
            if diag_counter >= 4 {
                return Some(last_flipped_bit.1);
            }
            
        }
        else {
            println!("ah");
        }

        None
    }

    fn run(&self) {
        loop {
            // user play()
            if let Some(color) = self.check_win() {
                // color won
                break;
            }
        }
    }
}

#[derive(Default)]
pub struct Perft {
    game: Game,
    tt: HashMap<(u64, i32), u64>
}

impl Perft {
    pub fn new() -> Self {
        Self { ..Default::default() }
    }
    
    pub fn run(&mut self, depth: i32) -> u64 {
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