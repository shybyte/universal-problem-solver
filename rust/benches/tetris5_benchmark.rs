use criterion::{criterion_group, criterion_main, Criterion};
use universal_problem_solver_rs::solve;
use universal_problem_solver_rs::example_problems::tetris5::{
    PuzzleProblem,
};
use universal_problem_solver_rs::example_problems::puzzle_pieces::{
    SKEW,
    SKEW2,
    STRAIGHT_PIECE,
    T_PIECE,
    L_REAL_PIECE,
    L_PIECE
};
use std::time::Duration;


fn solve_4_puzzle() {
    let problem = PuzzleProblem::new(4, 4, vec![
        SKEW.clone(),
        L_PIECE.clone(),
        L_REAL_PIECE.clone(),
        STRAIGHT_PIECE.clone()
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

fn solve_5_puzzle() {
    let problem = PuzzleProblem::new(5, 4, vec![
        SKEW.clone(),
        L_REAL_PIECE.clone(),
        T_PIECE.clone(),
        T_PIECE.clone(),
        STRAIGHT_PIECE.clone()
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

fn solve_10_puzzle() {
    let problem = PuzzleProblem::new(5, 8, vec![
        SKEW.clone(),
        SKEW.clone(),
        SKEW2.clone(),
        SKEW2.clone(),
        L_PIECE.clone(),
        L_REAL_PIECE.clone(),
        T_PIECE.clone(),
        T_PIECE.clone(),
        T_PIECE.clone(),
        T_PIECE.clone(),
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tetris5-4-5-pieces");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("solve 4 tetris puzzle", |b| b.iter(|| solve_4_puzzle()));
    group.bench_function("solve 5 tetris puzzle", |b| b.iter(|| solve_5_puzzle()));
    group.finish();

    let mut group_slow = c.benchmark_group("tetris5-10-pieces");
    group_slow.sample_size(50);
    group_slow.bench_function("10 piece puzzle", |b| b.iter(|| solve_10_puzzle()));
    group_slow.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);