use criterion::{Criterion, criterion_group, criterion_main};
use ux::u42;
use std::hint::black_box;

use puissance_4::api::{engine::Game, search::Search};

pub fn test_nets(depth1: i32, depth2: i32) {
    let mut game = Game::new();
    let mut search1 = Search::new(depth1);
    let mut search2 = Search::new(depth2);
    loop {
        if let Some(best_move) = search1.think(&mut game) {
            game.make_push(best_move);
        }

        if let Some(best_move) = search2.think(&mut game) {
            game.make_push(best_move);
        }

        if game.winner.is_some() || game.board.bitboard == u42::MAX {
            break;
        }
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("test nets");
    group.sample_size(10);

    // c.bench_function("depth 7", |b| b.iter(|| perft(black_box(7))));
    group.bench_function("Negamax 8 vs Negamax 8", |b| {
        b.iter(|| test_nets(black_box(8), black_box(8)))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
