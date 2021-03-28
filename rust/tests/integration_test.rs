use universal_problem_solver_rs::{Problem, solve};

#[derive(PartialEq, Debug, Copy, Clone)]
struct CalcState {
    number: i8
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum CalcAction {
    Plus1,
    Minus1,
}

#[derive(PartialEq, Debug, Copy, Clone)]
struct CalcProblem {
    initial_state: CalcState,
    goal_state: CalcState,
}

impl CalcProblem {
    pub fn new(initial_state: i8, goal_state: i8) -> Self {
        CalcProblem {
            initial_state: CalcState { number: initial_state },
            goal_state: CalcState { number: goal_state },
        }
    }
}


impl Problem<CalcState, CalcAction> for CalcProblem {
    fn get_initial_state(&self) -> CalcState {
        self.initial_state
    }

    fn get_actions(&self, _state: &CalcState) -> Vec<CalcAction> {
        vec![CalcAction::Minus1, CalcAction::Plus1]
    }

    fn apply_action(&self, state: &CalcState, action: &CalcAction) -> CalcState {
        match action {
            CalcAction::Plus1 => CalcState { number: state.number + 1 },
            CalcAction::Minus1 => CalcState { number: state.number - 1 }
        }
    }

    fn is_success(&self, state: &CalcState) -> bool {
        state.number == self.goal_state.number
    }
}


#[test]
fn test_solve_1_step() {
    let problem = CalcProblem::new(5, 6);
    let solution = solve(&problem, 1);

    assert_eq!(solution.len(), 1);
}

#[test]
fn test_solve_2_step() {
    let problem = CalcProblem::new(5, 7);

    let solution = solve(&problem, 1);
    assert_eq!(solution.len(), 0);

    let solution = solve(&problem, 2);
    assert_eq!(solution.len(), 2);
    eprintln!("solution = {:?}", solution);
}