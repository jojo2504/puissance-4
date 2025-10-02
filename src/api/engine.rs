use core::fmt;
use rand::seq::{IndexedMutRandom, SliceRandom};
use rand::rng;

#[derive(Clone, PartialEq, Eq, Copy)]
pub enum Color {
    Red,
    Yellow,
    None
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
    height: i32,
    width: i32,
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
        if col < 1 || col > self.width - 1 {
            println!("invalid input");
            return;
        }

        let mut row = self.height - 1;
        while self.board[row as usize][(col - 1) as usize] != Color::None {
            row -= 1;
        }
        self.board[row as usize][(col - 1) as usize] = token;
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
    board: Board,
    turn_color: Color
}

impl Game {
    pub fn new(height: i32, width: i32) -> Self {
        let mut available_colors = vec![Color::Yellow, Color::Red];
        let color = available_colors.choose_mut(&mut rand::rng());
        Self { turn_color: *color.unwrap(), board: Board::new(height, width)}
    }

    pub fn get_winner(&self) {
        // check diag

        // check rows
        
        // check cols
    }
}
