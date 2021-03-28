export interface Problem<ProblemState, Action> {
  getInitialState(): ProblemState;
  getActions(state: ProblemState): Action[];
  applyAction(state: ProblemState, action: Action): ProblemState;
  isSuccess(state: ProblemState): boolean;
}

export interface SolutionStep<ProblemState, Action> {
  state: ProblemState;
  action: Action;
}

export function solveProblem<ProblemState, Action>(
  problem: Problem<ProblemState, Action>,
  maxSteps: number
): Array<SolutionStep<ProblemState, Action>> {
  function solveProblemInternal(
    steps: Array<SolutionStep<ProblemState, Action>>,
    state: ProblemState
  ): Array<SolutionStep<ProblemState, Action>> {
    for (const action of problem.getActions(state)) {
      const nextState = problem.applyAction(state, action);
      steps.push({state: nextState, action: action});
      // console.log('nextState', state, action, nextState);
      if (problem.isSuccess(nextState)) {
        return steps;
      } else if (steps.length < maxSteps) {
        const childResult = solveProblemInternal(steps, nextState);
        if (childResult.length > 0) {
          return childResult;
        }
      }
      steps.pop();
    }
    return [];
  }

  return solveProblemInternal([], problem.getInitialState());
}
