use crate::api::engine::Color;

mod api;
mod gui;

fn main() {
    let mut board = api::engine::Board::new(5, 10);

    board.push(2, Color::Red);
    board.display_board();
}
