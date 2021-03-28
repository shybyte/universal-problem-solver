import * as _ from 'lodash';
import {solveProblem} from '../src';
import {lPiece, lRealPiece, PuzzleProblem, rotateMatrix, skew, straightPiece} from '../src/example-problems/tetris';


describe('solveProblem tetris', () => {
  test('1 step solution', () => {
    const problem = new PuzzleProblem(1, 1, [{matrix: [[1]]}]);

    const initialState = problem.getInitialState();
    console.log('initialState', initialState);

    const initialActions = problem.getActions(initialState);
    console.log('initialActions', initialActions);

    const nextState = problem.applyAction(initialState, initialActions[0]);
    console.log('nextState', nextState);

    console.log('problem.isSuccess(nextState)', problem.isSuccess(nextState));

    const solution = solveProblem(problem, 1);
    console.log('solution', JSON.stringify(solution, null, 2));
  });

  test('2 step solution', () => {
    const problem = new PuzzleProblem(1, 2, [{matrix: [[1]]}, {matrix: [[1]]}]);
    const solution = solveProblem(problem, 2);
    console.log('solution', JSON.stringify(solution, null, 2));
  });

  test('1 step solution rotation', () => {
    const problem = new PuzzleProblem(2, 1, [{matrix: [[1, 1]]}]);

    const initialState1 = problem.getInitialState();
    const actions = problem.getActions(initialState1);

    const solution = solveProblem(problem, 1);
    console.log('solution', JSON.stringify(solution, null, 2));
  });


  test('3 step solution rotation', () => {
    const problem = new PuzzleProblem(3, 4, [skew, lPiece, lPiece]);
    const solution = solveProblem(problem, 3);
    // console.log('solution', JSON.stringify(solution, null, 2));
    console.log('solution', '\n' + problem.prettifyState(_.last(solution)!.state));
  });

  test('4 step solution rotation', () => {
    const problem = new PuzzleProblem(4, 4, [skew, lPiece, lRealPiece, straightPiece]);
    const solution = solveProblem(problem, 4);
    // console.log('solution', JSON.stringify(solution, null, 2));
    console.log('solution', '\n' + problem.prettifyState(_.last(solution)!.state));
  });

  test('rotate straight', () => {
    const rotatePiece1 = rotateMatrix([[1, 1, 1, 1]]);
    console.log('rotateMatrix', rotatePiece1);
    const rotatePiece2 = rotateMatrix(rotatePiece1);
    console.log('rotateMatrix', rotatePiece2);
  });

  test('rotate skew', () => {
    console.log('skew', skew);
    const rotatePiece1 = rotateMatrix(skew.matrix);
    console.log('rotateMatrix', rotatePiece1);
    const rotatePiece2 = rotateMatrix(rotatePiece1);
    console.log('rotateMatrix', rotatePiece2);
    expect(rotatePiece2).toEqual(skew.matrix);
  });

});
