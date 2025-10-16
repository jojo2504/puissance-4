use core::{fmt};
use ux::u42;

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
            File::FileA => write!(f, "Base 2 (binary): {:042b}", File::FileA as u64),
            File::FileB => write!(f, "Base 2 (binary): {:042b}", File::FileB as u64),
            File::FileC => write!(f, "Base 2 (binary): {:042b}", File::FileC as u64),
            File::FileD => write!(f, "Base 2 (binary): {:042b}", File::FileD as u64),
            File::FileE => write!(f, "Base 2 (binary): {:042b}", File::FileE as u64),
            File::FileF => write!(f, "Base 2 (binary): {:042b}", File::FileF as u64),
            File::FileG => write!(f, "Base 2 (binary): {:042b}", File::FileG as u64),
        }
    }
}

impl TryFrom<i32> for File {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            0 => Ok(File::FileA),
            1 => Ok(File::FileB),
            2 => Ok(File::FileC),
            3 => Ok(File::FileD),
            4 => Ok(File::FileE),
            5 => Ok(File::FileF),
            6 => Ok(File::FileG),
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
    history: Vec<u42> // just keep the flipped bit in history
}

fn get_msb(bitboard: u42) -> Option<u42> {
    if bitboard == u42::new(0) {
        return None;
    }

    let mut index = u42::new(0x20000000000); // represent 1 + 41 0 in binary
    loop {
        if index & bitboard == u42::new(0) {
            index >>= 1;
        }
        else {
            return Some(index);
        }
    }
}

impl Board {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    fn push_helper(bitboard: &mut u42, color_bitboard: &mut u42, col: i32, history: &mut Vec<u42>) {
        if let Ok(file_mask) = File::try_from(col) {
            if *bitboard & u42::new(file_mask as u64) != u42::new(file_mask as u64) { // found that the col is not full
                if let Some(place_index) = get_msb(*bitboard) {
                    *color_bitboard ^= place_index << 7;
                    *bitboard ^= place_index << 7;
                    history.push(place_index << 7);
                }
                else {
                    let default_index= u42::new(1 << col);
                    *color_bitboard ^= default_index;
                    *bitboard ^= default_index;
                    history.push(default_index);
                }
            }
        }
    }

    pub fn push(&mut self, col: i32, token: Color) {
        match token {
            Color::Red => {
                Self::push_helper(&mut self.bitboard, &mut self.red, col, &mut self.history);
            },
            Color::Yellow => {
                Self::push_helper(&mut self.bitboard, &mut self.yellow, col, &mut self.history);
            }
        }
    }

    pub fn display_board(&self) {
        let binary_board = format!("{:042b}", self.bitboard);
        println!("{}", binary_board);
        
        for row in (0..6).rev() {
            for col in 0..7 {
                let index = row * 7 + col;
                if u42::new(1 << index) & self.red != u42::new(0) {
                    print!("{} ", 'R');
                }
                else if u42::new(1 << index) & self.yellow != u42::new(0) {
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

    pub fn get_winner(&self) -> Color {
        if let Some(last_flipped_bit) = self.board.history.last() {
            // horizontal check _
            last_flipped_bit, last_flipped_bit >> 1, last_flipped_bit >> 2, last_flipped_bit >> 3,  
            // vertical check |
            // main diagonal check \
            // anti diagonal check /
        }

        self.turn_color
    }
}
