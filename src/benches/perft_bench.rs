use criterion::{black_box, criterion_group, criterion_main, Criterion};
use crate::{api::engine::{Color, File, Game, Perft}, gui::play};

fn perft_benchmark(c: &mut Criterion) {
    c.bench_function("perft depth 5", |b| {
        b.iter(|| {
            let mut perft = Perft::new();
            black_box(perft.run(5))
        });
    });
    
    c.bench_function("perft depth 7", |b| {
        b.iter(|| {
            let mut perft = Perft::new();
            black_box(perft.run(7))
        });
    });
}

criterion_group!(benches, perft_benchmark);
criterion_main!(benches);