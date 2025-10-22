use puissance_4::{api::{engine::{Color, File, Game, Perft}, search::{Evaluation, Search}}, gui::play};
use rand::Rng;
use ux::u42;

fn main() {
    //let mut game = Game::new();
    //game.run();
    
    Search::test_nets(11, 11);
    Search::test_nets(5, 9);
    Search::test_nets(5, 2);
    Search::test_nets(4, 7);
}
