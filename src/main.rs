use puissance_4::{api::{engine::{Color, File, Game, Perft}, search::{Evaluation, Search}}, gui::play};
use rand::Rng;
use ux::u42;

fn main() {
    let mut game = Game::new();
    //game.run();

    // game.make_push_bulk("50500101010"); // win by red vertical
    // game.make_push_bulk("111522234443"); // win by yellow horizontal
    // game.make_push_bulk("01122363433"); // win by red diag /

    // game.make_push_bulk("32211210000"); // win by red diag \
    
    // game.make_push_bulk("4341110000"); // no win \
    
    // game.make_push_bulk("33335225644441430066655534665");
    
    // game.make_push_bulk("112263364400");
    // game.make_push_bulk("6140166111015");
    // println!("board evaluation: {}", Evaluation::evaluate(&game.board));
    // game.board.display_board();
    // game.test_bulk("0011223");
    // game.test_bulk("6140166111015");
    // game.test_bulk("6655443");
    game.test_bulk("23365636636535655531210112221140");

    // let mut move_history = String::new();
    // loop {
    //     if let Some(best_move) = Search::think(&mut game) {
    //         game.make_push(best_move);
    //         println!("board evaluation: {}", Evaluation::evaluate(&game.board));
    //         move_history += &best_move.to_string();
    //         game.board.display_board();
    //         println!();
    //     }
    //     else {
    //         match game.winner {
    //             Some(Color::Red) => println!("red won"),
    //             Some(Color::Yellow) => println!("yellow won"),
    //             _ => println!("no winner ?")
    //         }
    //         println!("{}", move_history);
    //         break;
    //     }
    // }

    // false positive:

    /* 
    51033231214553240423536
    02603611106214353021
    6140166111015
    */ 

    // loop {
    //     let num = rand::rng().random_range(0..7);
    //     if game.get_possible_moves().contains(&num) {
    //         game.make_push(num);
    //         move_history += &num.to_string();
    //         game.board.display_board();
    //         println!();

    //         match game.winner {
    //             Some(Color::Red) => {
    //                 println!("red won");
    //                 break;
    //             }
    //             Some(Color::Yellow) => {
    //                 println!("yellow won");
    //                 break;
    //             }
    //             _ => ()
    //         }
    //     }
    //     else {
    //         continue;
    //     }
    // }

    // println!("{}", move_history);

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
