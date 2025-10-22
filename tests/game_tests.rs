use puissance_4::api::engine::*;
use rstest::rstest;

pub fn test_bulk(game: &mut Game, history: &str) -> Option<Color> {
    game.make_push_bulk(history);
    game.board.display_board();
    println!("{}", history);

    game.winner
}

#[rstest]
#[case("50500101010", Some(Color::Red))] // vertical
#[case("6655443", Some(Color::Red))]
#[case("111522234443", Some(Color::Yellow))] //horizontal
#[case("3334344433655", Some(Color::Red))]
#[case("6140166111015", None)]
#[case("23365636636535655531210112221140", None)]
#[case("112263364400", None)]
#[case("45441432344223333210220550606", None)]
#[case("3333544454344223066666611111105", Some(Color::Red))]
fn test_alignment(#[case] history: &str, #[case] expected: Option<Color>) {
    let mut game = Game::new();
    assert_eq!(test_bulk(&mut game, history), expected);
}