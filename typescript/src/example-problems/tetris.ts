import * as _ from 'lodash';
import {Problem} from '../index';

export interface PuzzleState {
  playfield: number[][];
  pieceCount: number;
}

export interface PuzzleAction {
  pieceIndex: number;
  rotation: number;
  row: number;
  column: number;
}

export interface PuzzlePiece {
  matrix: number[][];
}

export class PuzzleProblem implements Problem<PuzzleState, PuzzleAction> {
  public readonly piecesRotations: PuzzlePiece[][];

  constructor(
    private readonly rows: number,
    private readonly cols: number,
    public readonly pieces: PuzzlePiece[]
  ) {
    this.piecesRotations = pieces.map(piece => {
      let currentMatrix = piece.matrix;
      return [piece].concat(_.times(3, () => {
        currentMatrix = rotateMatrix(currentMatrix);
        return {matrix: currentMatrix};
      }));
    })
  }

  applyAction(state: PuzzleState, action: PuzzleAction): PuzzleState {
    const nextState = _.cloneDeep(state);
    nextState.pieceCount += 1;

    const puzzlePiece = this.piecesRotations[action.pieceIndex][action.rotation];
    puzzlePiece.matrix.forEach((row, rowIndex) => {
      row.forEach((cell, colIndex) => {
        const targetRow = action.row + rowIndex;
        const targetCol = action.column + colIndex;
        if (cell > 0 && targetRow < nextState.playfield.length && targetCol < nextState.playfield[targetRow].length) {
          nextState.playfield[targetRow][targetCol] = action.pieceIndex;
        }
      });
    });

    return nextState;
  }

  getActions(state: PuzzleState): PuzzleAction[] {
    const actions: PuzzleAction[] = []
    state.playfield.forEach((row, rowIndex) => {
      row.forEach((_cell, colIndex) => {
        actions.push(...this.piecesRotations[state.pieceCount].map((_rotated, rotationIndex) => ({
          column: colIndex,
          row: rowIndex,
          pieceIndex: state.pieceCount,
          rotation: rotationIndex
        })));
      });
    });
    return actions;
  }

  getInitialState(): PuzzleState {
    return {
      pieceCount: 0,
      playfield: _.times(this.rows).map(() => new Array(this.cols).fill(-1))
    };
  }

  isSuccess(state: PuzzleState) {
    return state.playfield.every(row => row.every(it => it > -1));
  }

  prettifyState(state: PuzzleState): string {
    return state.playfield.map(row => row.join('')).join('\n');
  }
}

export function rotateMatrix(matrix: number[][]) {
  return matrix[0].map((val, index) => matrix.map(row => row[index]).reverse())
}

export const skew = {
  matrix: [
    [1, 1, 0],
    [0, 1, 1]
  ]
};

export const lPiece = {
  matrix: [
    [1, 0, 0],
    [1, 1, 1]
  ]
};

export const lRealPiece = {
  matrix: [
    [0, 0, 1],
    [1, 1, 1]
  ]
};

export const straightPiece = {
  matrix: [
    [1, 1, 1, 1]
  ]
};
