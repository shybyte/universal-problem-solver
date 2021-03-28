use universal_problem_solver_rs::solve;
use universal_problem_solver_rs::example_problems::tetris5::{
    PuzzleProblem,
};
use universal_problem_solver_rs::example_problems::puzzle_pieces::{
    PuzzlePiece,
    SKEW,
    SKEW2,
    STRAIGHT_PIECE,
    L_REAL_PIECE,
    L_PIECE,
    T_PIECE
};

#[test]
fn test_solve_1_step() {
    let problem = PuzzleProblem::new(1, 1, vec![PuzzlePiece { matrix: vec![vec![1]] }]);
    let solution = solve(&problem, 1);

    eprintln!("solution = {:?}", solution);
    assert_eq!(solution.len(), 1);
}

#[test]
fn test_solve_2_step() {
    let problem = PuzzleProblem::new(1, 2, vec![
        PuzzlePiece { matrix: vec![vec![1]] },
        PuzzlePiece { matrix: vec![vec![1]] }
    ]);

    let solution = solve(&problem, 2);
    eprintln!("solution = {:?}", solution);
    assert_eq!(solution.len(), 2);
}

#[test]
fn test_solve_2_step_vertical() {
    let problem = PuzzleProblem::new(2, 1, vec![
        PuzzlePiece { matrix: vec![vec![1]] },
        PuzzlePiece { matrix: vec![vec![1]] }
    ]);

    let solution = solve(&problem, 2);
    eprintln!("solution = {:?}", solution);
    assert_eq!(solution.len(), 2);
}

#[test]
fn test_solve_1_step_with_rotation() {
    let problem = PuzzleProblem::new(2, 1, vec![
        PuzzlePiece { matrix: vec![vec![1, 1]] }
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    eprintln!("solution = {:?}", solution);
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

#[test]
fn test_solve_3_step_with_rotation() {
    let problem = PuzzleProblem::new(3, 4, vec![
        L_PIECE.clone(),
        SKEW.clone(),
        L_PIECE.clone(),
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    eprintln!("solution = \n{:}", problem.prettify_solution(&solution));
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

#[test]
fn test_solve_4_step_with_rotation() {
    let problem = PuzzleProblem::new(4, 4, vec![
        SKEW.clone(),
        L_PIECE.clone(),
        L_REAL_PIECE.clone(),
        STRAIGHT_PIECE.clone()
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    eprintln!("solution = \n{:}", problem.prettify_state(&solution.iter().last().unwrap().state));
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

// Gate B
#[test]
fn test_solve_5_pieces() {
    let problem = PuzzleProblem::new(5, 4, vec![
        SKEW.clone(),
        L_REAL_PIECE.clone(),
        T_PIECE.clone(),
        T_PIECE.clone(),
        STRAIGHT_PIECE.clone()
    ]);

    let solution = solve(&problem, problem.pieces_rotations.len());
    eprintln!("solution = \n{:}", problem.prettify_state(&solution.iter().last().unwrap().state));
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}

// Star Gate A https://steamcommunity.com/sharedfiles/filedetails/?id=354590899
#[test]
fn test_solve_10_pieces() {
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
    eprintln!("solution = \n{:}", problem.prettify_solution(&solution));
    assert_eq!(solution.len(), problem.pieces_rotations.len());
}



