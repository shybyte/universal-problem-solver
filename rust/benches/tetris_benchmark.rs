use criterion::{criterion_group, criterion_main, Criterion};
use universal_problem_solver_rs::solve;
use universal_problem_solver_rs::example_problems::tetris::{
    PuzzleProblem,
};
use universal_problem_solver_rs::example_problems::puzzle_pieces::{
    SKEW,
    STRAIGHT_PIECE,
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

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("tetris-slow");
    group.measurement_time(Duration::from_secs(10));
    group.bench_function("solve 4 tetris puzzle", |b| b.iter(|| solve_4_puzzle()));
    group.finish();

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);