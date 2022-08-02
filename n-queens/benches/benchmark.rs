use criterion::{black_box, criterion_group, criterion_main, Criterion};
use n_queens::Solution;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("n100", |b| b.iter(|| Solution::solve_n_queens(black_box(10))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
