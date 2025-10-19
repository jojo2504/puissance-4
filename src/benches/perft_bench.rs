use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};

use puissance_4::api::engine::Perft;

fn perft(depth: i32) -> u64 {
    let mut perft = Perft::new();
    perft.run(depth)
}
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("depth 7", |b| b.iter(|| perft(black_box(7))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);