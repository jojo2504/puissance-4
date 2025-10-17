
use crate::{api::engine::{Color, File, Game}, gui::play};

mod api;
mod gui;

fn main() {
    let mut game = Game::new();
    
    game.board.make_push(0, Color::Red);
    game.board.display_board();
    game.board.make_push(0, Color::Yellow);
    game.board.display_board();
    game.board.unmake_push();
    game.board.display_board();
}
