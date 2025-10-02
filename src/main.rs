use crate::{api::engine::{Color, Game}, gui::play};

mod api;
mod gui;

fn main() {
    let mut game = Game::new(6, 7);

    loop {
        let col = play::play(&game);
        if game.turn_color == Color::Red {
            game.board.push(col, game.turn_color);
            game.turn_color = Color::Yellow
        }
        else {
            game.board.push(col, game.turn_color);
            game.turn_color = Color::Red
        }

        game.board.display_board();
    }
}
