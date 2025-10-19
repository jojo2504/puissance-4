use puissance_4::{api::engine::{Color, File, Game, Perft}, gui::play};
use colored::Colorize;

fn main() {
    // let mut game = Game::new();

    // println!("{}", game.turn_color);

    // game.make_push(0);
    // game.make_push(0);
    // game.unmake_push();

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
    for i in 1..10 {
        perft.reset();
        let perft_result = perft.run(i);
        perft.reset();
        let perft_tt_result = perft.run_tt(i);
        
        println!("perft_result: {}", perft_result);
        println!("perft_tt_result: {}", perft_tt_result);
        assert_eq!(perft_result, perft_tt_result);
        println!("{}\n", format!("perft {} passed!", i).green());
    }

}
