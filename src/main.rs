
use crate::{api::engine::{Color, File, Game}, gui::play};

mod api;
mod gui;

fn main() {
    let mut game = Game::new();
    
    game.board.push(0, Color::Red);
    game.board.display_board();
    game.board.push(0, Color::Red);
    game.board.display_board();
}
