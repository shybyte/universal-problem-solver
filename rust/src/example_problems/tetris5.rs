use std::fmt::Debug;

use itertools::join;

use crate::{Problem, SolutionStep};
use crate::example_problems::bit_set_64::BitSet64;
use crate::example_problems::puzzle_pieces::PuzzlePiece;

#[derive(PartialEq, Clone, Debug)]
pub struct PuzzleState {
    pub playfield: BitSet64,
    pub piece_count: u8,
}

#[derive(PartialEq, Debug, Clone)]
pub struct PuzzleAction {
    piece_index: u8,
    rotation: u8,
    row: u8,
    column: u8,
    mask: BitSet64,
}

#[derive(PartialEq, Debug, Clone)]
pub struct PuzzleProblem {
    pub pieces_rotations: Vec<Vec<PuzzlePiece>>,
    pub pieces_actions: Vec<Vec<PuzzleAction>>,
    rows: usize,
    cols: usize,
    filled_playfield: BitSet64,
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
                    if rotations.iter().all(|it| !it.matrix.eq(&current_matrix)) {
                        rotations.push(PuzzlePiece { matrix: current_matrix.clone() });
                    }
                }
                rotations
            })
            .collect();

        let mut result = PuzzleProblem {
            cols,
            rows,
            pieces_rotations,
            pieces_actions: vec![],
            filled_playfield: BitSet64::new_filled((cols * rows) as u8),
        };

        result.pieces_actions = (0..pieces.len()).map(|piece_index| result.get_actions(piece_index)).collect();

        result
    }

    fn get_actions(&self, piece_index: usize) -> Vec<PuzzleAction> {
        let mut actions = vec![];

        for row in 0..self.rows {
            for col in 0..self.cols {
                for rotation in 0..self.pieces_rotations[piece_index].len() {
                    let puzzle_piece_matrix = &self.pieces_rotations[piece_index][rotation].matrix;
                    if row + puzzle_piece_matrix.len() <= self.rows && col + puzzle_piece_matrix[0].len() <= self.cols {
                        actions.push(PuzzleAction {
                            piece_index: piece_index as u8,
                            column: col as u8,
                            row: row as u8,
                            rotation: rotation as u8,
                            mask: self.create_action_mask(piece_index as u8, rotation as u8, row, col),
                        });
                    }
                }
            }
        }

        return actions;
    }


    fn is_action_legal(&self, state: &PuzzleState, action: &PuzzleAction) -> bool {
        !state.playfield.and(&action.mask).any()
    }

    fn playfield_index(&self, row: u8, col: u8) -> u8 {
        row * self.cols as u8 + col
    }

    fn create_action_mask(
        &self, piece_index: u8,
        rotation: u8,
        row: usize,
        column: usize,
    ) -> BitSet64 {
        let mut result = BitSet64::new(0);

        let puzzle_piece = &self.pieces_rotations[piece_index as usize][rotation as usize];
        for (row_index, piece_row) in puzzle_piece.matrix.iter().enumerate() {
            for (col_index, cell) in piece_row.iter().enumerate() {
                if *cell > 0 {
                    result.set(self.playfield_index(row as u8 + row_index as u8, column as u8 + col_index as u8));
                }
            }
        }

        return result;
    }

    pub fn prettify_state(&self, state: &PuzzleState) -> String {
        format!("{:?}", state)
    }

    pub fn prettify_solution(&self, solution: &Vec<SolutionStep<PuzzleState, PuzzleAction>>) -> String {
        let mut playfield: Vec<Vec<i8>> = vec![vec![-1; self.cols]; self.rows];

        for SolutionStep { action, .. } in solution {
            let puzzle_piece = &self.pieces_rotations[action.piece_index as usize][action.rotation as usize];
            for (row_index, row) in puzzle_piece.matrix.iter().enumerate() {
                for (col_index, cell) in row.iter().enumerate() {
                    if *cell > 0 {
                        playfield[action.row as usize + row_index][action.column as usize + col_index] = action.piece_index as i8;
                    }
                }
            }
        }

        join(playfield.iter().map(|row| join(row, "")), "\n")
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
            playfield: BitSet64::new(0),
        }
    }

    fn get_actions(&self, state: &PuzzleState) -> Vec<PuzzleAction> {
        self.pieces_actions[state.piece_count as usize].iter()
            .filter(|it| self.is_action_legal(&state, *it))
            .cloned().collect()
    }

    fn apply_action(&self, state: &PuzzleState, action: &PuzzleAction) -> PuzzleState {
        let mut next_state = state.clone();
        next_state.piece_count += 1;
        next_state.playfield = next_state.playfield.or(&action.mask);
        return next_state;
    }

    fn is_success(&self, state: &PuzzleState) -> bool {
        state.playfield == self.filled_playfield
    }
}

