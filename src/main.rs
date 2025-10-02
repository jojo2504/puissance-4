mod api;
mod gui;

fn main() {
    let mut board = api::engine::Board::new(5, 10);

    board.push(2);
    board.display_board();
}
