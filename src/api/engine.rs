use core::fmt;
use rand::seq::{IndexedMutRandom, SliceRandom};
use rand::rng;
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

pub struct Board {
    pub height: i32,
    pub width: i32,
    board: Vec<Vec<Color>>,
    history: Vec<(i32, i32)> // row, col
}

impl Board {
    pub fn new(height: i32, width: i32) -> Self {
        Self {
            height: height,
            width: width,
            board: vec![vec![Color::None; width as usize]; height as usize],
            history: vec![]
        }
    }

    pub fn push(&mut self, col: i32, token: Color) {
        if col < 1 || col > self.width {
            println!("invalid input");
            return;
        }

        let mut row = self.height - 1;
        while self.board[row as usize][(col - 1) as usize] != Color::None {
            row -= 1;
            if row == -1 {
                println!("invalid input");
                return;
            }
        }
        self.board[row as usize][(col - 1) as usize] = token;
        self.history.push((col - 1, row));
    }

    pub fn display_board(&self) {
        print!("{}", "--------".repeat(self.board.len()));
        println!();
        for i in 0..self.board.len() { // height
            for j in 0..self.board[0].len() { // width
                print!("| {} ", self.board[i][j]);
            }
            println!("|");
            print!("{}", "--------".repeat(self.board.len()));
            println!();
        }
    }

}

pub struct Game {
    pub board: Board,
    pub turn_color: Color
}

impl Game {
    pub fn new(height: i32, width: i32) -> Self {
        let mut available_colors = vec![Color::Yellow, Color::Red];
        let color = available_colors.choose_mut(&mut rand::rng());
        Self { turn_color: *color.unwrap(), board: Board::new(height, width)}
    }

    pub fn get_winner(&self) -> Color {
        let last_entry = self.board.history.last().unwrap();
        let current_x = last_entry.0;
        let current_y = last_entry.1;
        let current_token = &self.board.board[current_y as usize][current_x as usize];

        let mut is_win: bool = true;
        let mut i = 0;

        // COLUMNS
        // Right direction
        for x in current_x..cmp::min(current_x+4, self.board.width) {
            let token = &self.board.board[current_y as usize][x as usize];
            if token.eq(&Color::None) {
                break;
            }

            if !token.eq(current_token) {
                is_win = false;
                break;
            }

            i += 1;
        }
        if is_win && i == 3 {
            return current_token.clone();
        }

        // Left direction
        i = 0;
        is_win = true;
        for x in Iterator::rev(cmp::max(current_x-4, 0)..current_x) {
            let token: &Color = &self.board.board[current_y as usize][x as usize];
            if token.eq(&Color::None) {
                break;
            }

            if !token.eq(current_token) {
                is_win = false;
                break;
            }

            i += 1;
        }
        if is_win && i == 3 {
            return current_token.clone();
        }

        // ROWS
        // Bottom direction
        i = 0;
        is_win = true;
        for y in current_y..cmp::min(current_y+4, self.board.height) {
            let token = &self.board.board[y as usize][current_x as usize];
            if token.eq(&Color::None) {
                break;
            }
            
            if !token.eq(current_token) {
                is_win = false;
                break;
            }
            i += 1;
        }
        if is_win && i == 3 {
            return current_token.clone();
        }

        // Top direction
        i = 0;
        is_win = true;
        for y in Iterator::rev(cmp::max(current_y-4, 0)..current_y) {
            let token = &self.board.board[y as usize][current_x as usize];
            if token.eq(&Color::None) {
                break;
            }

            if !token.eq(current_token) {
                is_win = false;
                break;
            }
            i += 1;
        }
        if is_win && i == 3 {
            return current_token.clone();
        }

        return Color::None;
    }
}
