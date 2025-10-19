use puissance_4::{api::{engine::{Color, File, Game, Perft}, search::{Evaluation, Search}}, gui::play};
use ux::u42;

fn main() {
    let mut game = Game::new();

    // game.make_push_bulk("50500101010"); // win by red vertical
    // game.make_push_bulk("111522234443"); // win by yellow horizontal
    // game.make_push_bulk("01122363433"); // win by red diag /

    // game.make_push_bulk("32211210000"); // win by red diag \
    
    // game.make_push_bulk("4341110000"); // no win \
    
    //game.make_push_bulk("01122323433");
    
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
    
    let mut move_history = String::new();
    loop {
        if let Some(best_move) = Search::think(&mut game) {
            game.make_push(best_move);
            println!("board evaluation: {}", Evaluation::evaluate(&game.board));
            move_history += &best_move.to_string();
            game.board.display_board();
            println!();
        }
        else {
            match game.winner {
                Some(Color::Red) => println!("red won"),
                Some(Color::Yellow) => println!("yellow won"),
                _ => println!("no winner ?")
            }
            println!("{}", move_history);
            break;
        }
    }

    // let mut perft = Perft::new();
    // for i in 1..10 {
    //     perft.reset();
    //     let perft_result = perft.run(i);
    //     perft.reset();
    //     let perft_tt_result = perft.run_tt(i);
        
    //     println!("perft_result: {}", perft_result);
    //     println!("perft_tt_result: {}", perft_tt_result);
    //     assert_eq!(perft_result, perft_tt_result);
    //     println!("{}\n", format!("perft {} passed!", i).green());
    // }

}
