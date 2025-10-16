use core::fmt;
use ux::u42;
use std::cmp;

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Color {
    Red,
    Yellow,
    None
}

impl Color {
    pub fn to_int(self) -> u8 {
        self as u8
    }
    
    pub fn from_int(value: u8) -> Option<Self> {
        match value {
            0 => Some(Color::Red),
            1 => Some(Color::Yellow),
            2 => Some(Color::None),
            _ => None,
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Color::Red => write!(f, "R"),
            Color::Yellow => write!(f, "Y"),
            Color::None => write!(f, " "),
        }
    }
}

pub enum File {
    FileA = 0x810204081,
    FileB = 0x1020408102,
    FileC = 0x2040810204,
    FileD = 0x4081020408,
    FileE = 0x8102040810,
    FileF = 0x10204081020,
    FileG = 0x20408102040,
}

impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            File::FileA => write!(f, "Base 2 (binary): {:042b}", File::FileA as isize),
            File::FileB => write!(f, "Base 2 (binary): {:042b}", File::FileB as isize),
            File::FileC => write!(f, "Base 2 (binary): {:042b}", File::FileC as isize),
            File::FileD => write!(f, "Base 2 (binary): {:042b}", File::FileD as isize),
            File::FileE => write!(f, "Base 2 (binary): {:042b}", File::FileE as isize),
            File::FileF => write!(f, "Base 2 (binary): {:042b}", File::FileF as isize),
            File::FileG => write!(f, "Base 2 (binary): {:042b}", File::FileG as isize),
        }
    }
}

pub struct Board {
    bitboard: u42 // board is 7 col x 6 rows, same encoding as a chess board; (0, 0) is bottom left, going to right, then up
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboard: u42::new(0)
        }
    }

    pub fn push(&mut self, col: i32, token: Color) {
        todo!()
    }

    pub fn display_board(&self) {
        let binary_board = format!("{:042b}", self.bitboard);
        let chars: Vec<char> = binary_board.chars().collect();
        
        for row in 0..6 {
            for col in 0..7 {
                let index = row * 7 + col;
                print!("{} ", chars[index]);
            }
            println!();
        }
    }

}

pub struct Game {
    pub board: Board,
    pub turn_color: Color
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            turn_color: Color::Yellow
        }
    }

    pub fn get_winner(&self) -> Color {
        todo!()
    }
}
