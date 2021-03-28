#[derive(PartialEq, Debug, Clone)]
pub struct PuzzlePiece {
    pub matrix: Vec<Vec<i8>>,
}

lazy_static! {
    pub static ref SKEW: PuzzlePiece = PuzzlePiece {
        matrix: vec![
            vec![1, 1, 0],
            vec![0, 1, 1]
        ]
    };

    pub static ref SKEW2: PuzzlePiece = PuzzlePiece {
        matrix: vec![
            vec![0, 1, 1],
            vec![1, 1, 0]
        ]
    };

    pub static ref L_PIECE: PuzzlePiece = PuzzlePiece {
        matrix: vec![
            vec![1, 0, 0],
            vec![1, 1, 1]
        ]
    };

    pub static ref L_REAL_PIECE: PuzzlePiece = PuzzlePiece {
        matrix: vec![
            vec![0, 0, 1],
            vec![1, 1, 1]
        ]
    };

    pub static ref T_PIECE: PuzzlePiece = PuzzlePiece {
        matrix: vec![
            vec![0, 1, 0],
            vec![1, 1, 1]
        ]
    };

    pub static ref STRAIGHT_PIECE: PuzzlePiece = PuzzlePiece {
        matrix: vec![
            vec![1, 1, 1, 1],
        ]
    };
}