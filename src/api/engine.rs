use core::{fmt};
use ux::u42;

const EMPTY_BOARD: u42 = u42::new(0);

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Color {
    Red,
    Yellow
}

impl Color {
    pub fn to_int(self) -> u8 {
        self as u8
    }
    
    pub fn from_int(value: u8) -> Option<Self> {
        match value {
            0 => Some(Color::Red),
            1 => Some(Color::Yellow),
            _ => None,
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
    return Some(u42::new(1) << (63 - raw.leading_zeros()));
}

impl Board {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn push(bitboard: &mut u42, color_bitboard: &mut u42, col: i32, history: &mut Vec<(u42, Color)>, token_color: Color) {
        if let Ok(file_mask) = File::try_from(col) {
            if *bitboard & u42::new(file_mask as u64) != u42::new(file_mask as u64) { // found that the col is not full
                if let Some(place_index) = get_msb(*bitboard) {
                    *color_bitboard ^= place_index << 7;
                    *bitboard ^= place_index << 7;
                    history.push((place_index << 7, token_color));
                }
                else {
                    let default_index= u42::new(1 << col);
                    *color_bitboard ^= default_index;
                    *bitboard ^= default_index;
                    history.push((default_index, token_color));
                }
            }
        }
    }

    pub fn make_push(&mut self, col: i32, token_color: Color) {
        match token_color {
            Color::Red => {
                Self::push(&mut self.bitboard, &mut self.red, col, &mut self.history, token_color);
            },
            Color::Yellow => {
                Self::push(&mut self.bitboard, &mut self.yellow, col, &mut self.history, token_color);
            }
        }
    }

    pub fn unmake_push(&mut self) {
        let last_play = self.history.pop().unwrap();
        self.bitboard ^= last_play.0;
        match last_play.1 {
            Color::Red => {
                self.red ^= last_play.0;
            },
            Color::Yellow => {
                self.yellow ^= last_play.0;
            }
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

pub struct Game {
    pub board: Board,
    turn_color: Color
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn_color: Color::Yellow
        }
    }

    fn check_win(&self) -> Option<Color> {
        if let Some(last_flipped_bit) = self.board.history.last() {
            // horizontal check _
            // vertical check |
            // main diagonal check \
            // anti diagonal check /
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
