use itertools::join;

use crate::{Problem};
use crate::example_problems::puzzle_pieces::PuzzlePiece;

#[derive(PartialEq, Debug, Clone)]
pub struct PuzzleState {
    pub playfield: Vec<Vec<i8>>,
    pub piece_count: u8,
}


#[derive(PartialEq, Debug, Copy, Clone)]
pub struct PuzzleAction {
    piece_index: u8,
    rotation: u8,
    row: usize,
    column: usize,
}

#[derive(PartialEq, Debug, Clone)]
pub struct PuzzleProblem {
    pub pieces_rotations: Vec<Vec<PuzzlePiece>>,
    rows: usize,
    cols: usize,
}

impl PuzzleProblem {
    pub fn new(
        rows: usize,
        cols: usize,
        pieces: Vec<PuzzlePiece>,
    ) -> Self {
        let pieces_rotations: Vec<Vec<PuzzlePiece>> = pieces.iter()
            .map(|piece| {
                let mut current_matrix = piece.matrix.clone();
                let mut rotations = vec![piece.clone()];
                for _index in 1..4 {
                    current_matrix = rotate_matrix(&current_matrix);
                    rotations.push(PuzzlePiece { matrix: current_matrix.clone() });
                }
                rotations
            })
            .collect();

        PuzzleProblem {
            cols,
            rows,
            pieces_rotations,
        }
    }

    pub fn prettify_state(&self, state: &PuzzleState) -> String {
        join(state.playfield.iter().map(|row| join(row, "")), "\n")
    }
}

fn rotate_matrix<T: Copy>(matrix: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    matrix[0].iter().enumerate().map(|(index, _value)|
        matrix.iter().map(|row| row[index]).rev().collect()
    ).collect()
}

impl Problem<PuzzleState, PuzzleAction> for PuzzleProblem {
    fn get_initial_state(&self) -> PuzzleState {
        PuzzleState {
            piece_count: 0,
            playfield: vec![vec![-1; self.cols]; self.rows],
        }
    }

    fn get_actions(&self, state: &PuzzleState) -> Vec<PuzzleAction> {
        let mut actions = vec![];

        for row in 0..self.rows {
            for col in 0..self.cols {
                for rotation in 0..self.pieces_rotations[state.piece_count as usize].len() {
                    actions.push(PuzzleAction {
                        piece_index: state.piece_count,
                        column: col,
                        row,
                        rotation: rotation as u8,
                    })
                }
            }
        }

        return actions;
    }

    fn apply_action(&self, state: &PuzzleState, action: &PuzzleAction) -> PuzzleState {
        let mut next_state = state.clone();
        next_state.piece_count += 1;

        let puzzle_piece = &self.pieces_rotations[action.piece_index as usize][action.rotation as usize];
        for (row_index, row) in puzzle_piece.matrix.iter().enumerate() {
            for (col_index, cell) in row.iter().enumerate() {
                let target_row = action.row + row_index;
                let target_col = action.column + col_index;
                if *cell > 0 && target_row < self.rows && target_col < self.cols {
                    next_state.playfield[target_row][target_col] = action.piece_index as i8;
                }
            }
        }

        return next_state;
    }

    fn is_success(&self, state: &PuzzleState) -> bool {
        state.playfield.iter().all(|row| row.iter().all(|cell| *cell > -1))
    }
}
