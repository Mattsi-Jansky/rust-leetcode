use criterion::{black_box, criterion_group, criterion_main, Criterion};
use valid_sudoku::Solution;

fn criterion_benchmark(c: &mut Criterion) {
    let incomplete = vec![
        vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
        vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
        vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
        vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
        vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
        vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
        vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
        vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
        vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
    ];
    let complete = vec![
        vec!['8', '2', '7', '1', '5', '4', '3', '9', '6'],
        vec!['9', '6', '5', '3', '2', '7', '1', '4', '8'],
        vec!['3', '4', '1', '6', '8', '9', '7', '5', '2'],
        vec!['5', '9', '3', '4', '6', '8', '2', '7', '1'],
        vec!['4', '7', '2', '5', '1', '3', '6', '8', '9'],
        vec!['6', '1', '8', '9', '7', '2', '4', '3', '5'],
        vec!['7', '8', '6', '2', '3', '5', '9', '1', '4'],
        vec!['1', '5', '4', '7', '9', '6', '8', '2', '3'],
        vec!['2', '3', '9', '8', '4', '1', '5', '6', '7'],
    ];

    c.bench_function("valid sudoku", |b| b.iter(
        || Solution::is_valid_sudoku(black_box(&complete)))
    );

    c.bench_function("valid sudoku", |b| b.iter(
        || Solution::is_valid_sudoku(black_box(&incomplete)))
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
