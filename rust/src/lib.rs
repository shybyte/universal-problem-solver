#[macro_use]
extern crate lazy_static;

use std::fmt::Debug;

pub mod example_problems;

pub trait Problem<ProblemState, Action> {
    fn get_initial_state(&self) -> ProblemState;
    fn get_actions(&self, state: &ProblemState) -> Vec<Action>;
    fn apply_action(&self, state: &ProblemState, action: &Action) -> ProblemState;
    fn is_success(&self, state: &ProblemState) -> bool;
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct SolutionStep<ProblemState, Action> {
    pub state: ProblemState,
    action: Action,
}


pub fn solve<ProblemState: Clone, Action, P>(
    problem: &P,
    max_steps: usize,
) -> Vec<SolutionStep<ProblemState, Action>>
    where
        P: Problem<ProblemState, Action>,
        ProblemState: Clone + Debug,
        Action: Debug
{
    let mut steps = vec![];
    solve_internally(problem, max_steps, &mut steps, &problem.get_initial_state());
    steps
}

pub fn solve_internally<ProblemState, Action, P>(
    problem: &P,
    max_steps: usize,
    steps: &mut Vec<SolutionStep<ProblemState, Action>>,
    state: &ProblemState,
) -> bool
    where
        P: Problem<ProblemState, Action>,
        ProblemState: Clone + Debug,
        Action: Debug
{
    for action in problem.get_actions(state) {
        let next_state = problem.apply_action(&state, &action);
        let step = SolutionStep { state: next_state.clone(), action };
        steps.push(step);
        // if steps.len() == 1 {
        //     eprintln!("steps = {:?}", steps.iter().last().unwrap().state);
        // }
        if problem.is_success(&next_state) {
            return true;
        } else if steps.len() < max_steps {
            if solve_internally(problem, max_steps, steps, &next_state) {
                return true;
            }
        }
        steps.pop();
    }
    false
}
