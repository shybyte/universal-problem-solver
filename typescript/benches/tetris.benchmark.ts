import { suite, benchmark } from "@dynatrace/zakzak";
import {solveProblem} from '../src';
import {lPiece, lRealPiece, PuzzleProblem, skew, straightPiece} from '../src/example-problems/tetris';

suite("solve tetris puzzle", () => {
  benchmark("4 step solution rotation", () => {
    const problem = new PuzzleProblem(4, 4, [skew, lPiece, lRealPiece, straightPiece]);
    const solution = solveProblem(problem, 4);
  });
});
