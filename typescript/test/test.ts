import {solveProblem, Problem} from '../src';

type CalcState = number;

enum CalcAction {
  plusOne = 'plusOne',
  minusOne = 'minusOne',
}

class CalcProblem implements Problem<CalcState,CalcAction> {
  constructor(public readonly initialState: CalcState, public readonly goalState: CalcState) {
  }

  applyAction(state: CalcState, action: CalcAction): CalcState {
    switch (action) {
      case CalcAction.minusOne:
        return state - 1;
      case CalcAction.plusOne:
        return state + 1;
      default:
        throw `Unsupported action "${action}"`;
    }
  }

  getActions(state: CalcState): CalcAction[] {
    return [CalcAction.minusOne, CalcAction.plusOne];
  }

  getInitialState(): CalcState {
    return this.initialState;
  }

  isSuccess(state: CalcState) {
    return state === this.goalState;
  }
}


describe('solveProblem', () => {
  test('1 step solution', () => {
    const calcProblem = new CalcProblem(5, 6);
    const solution = solveProblem(calcProblem, 1);
    expect(solution).toEqual([{state: 6, action: 'plusOne'}]);
  });

  test('2 step solution', () => {
    const calcProblem = new CalcProblem(5, 7);
    const solution = solveProblem(calcProblem, 2);
    expect(solution).toEqual([{state: 6, action: 'plusOne'}, {state: 7, action: 'plusOne'}]);
  });
});
