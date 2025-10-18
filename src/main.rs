
use crate::{api::engine::{Color, File, Game, Perft}, gui::play};

mod api;
mod gui;

fn main() {
    // let a = 16;
    // println!("{}", a / 7);
    // let mut game = Game::new();
    // game.board.make_push(3, Color::Red);
    // game.board.make_push(3, Color::Red);
    // game.board.make_push(3, Color::Red);
    // game.board.make_push(3, Color::Yellow);
    // game.board.make_push(2, Color::Red);
    // game.board.make_push(2, Color::Red);
    // game.board.make_push(2, Color::Yellow);
    // game.board.make_push(1, Color::Red);
    // game.board.make_push(1, Color::Yellow);
    // game.board.make_push(0, Color::Yellow);

    // game.board.display_board();

    // if let Some(color) = game.check_win() {
    //     match color {
    //         Color::Red => println!("red won"),
    //         Color::Yellow => println!("yellow won"),
    //     }
    // }
    // else {
    //     println!("no one won");
    // }

    let mut perft = Perft::new();
    println!("{}", perft.run(7));

}
