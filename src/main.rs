use crate::{api::engine::{Color, Game}, gui::play};

mod api;
mod gui;

fn main() {
    let mut game = Game::new(6, 7);

    loop {
        let col = play::play(&game);
        game.board.push(col, game.turn_color);
        game.turn_color = Color::from_int(Color::to_int(game.turn_color) ^ 1).unwrap();
        game.board.display_board();
        
        println!("{}", game.turn_color);
    }
}
