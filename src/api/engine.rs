pub struct Board {
    height: i32,
    width: i32,
    board: Vec<Vec<Option<char>>>
}

impl Board {
    pub fn new(height: i32, width: i32) -> Self {
        Self {
            height: height,
            width: width,
            board: vec![vec![Some('#'); width as usize]; height as usize]
        }
    }

    pub fn push(&mut self, col: i32) {
        if col < 1 || col > self.width-1 {
            println!("invalid input");
            return;
        }
        
        let mut row = self.height-1;
        while self.board[row as usize][(col-1) as usize].unwrap() != '#' {
            row -= 1;
        }
        self.board[row as usize][(col-1) as usize] = Some('S');
    }

    pub fn display_board(& self) {
        //println!("height is {}", self.height);
        //println!("width is {}", self.width);
        for i in 0..self.board.len() { // height
            for j in 0..self.board[0].len() { // width
                print!("{}", self.board[i][j].unwrap());
            }
            println!();
        }
    }
}

pub struct Game {

}

impl Game {

}