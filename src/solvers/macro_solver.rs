use radix_heap::RadixHeapMap;
use rustc_hash::FxHashSet;

use crate::game::{
    state::InProgress, units::Quality, Action, ActionMask, Condition, Settings, State,
};
use crate::solvers::actions::{
    DURABILITY_ACTIONS, MIXED_ACTIONS, PROGRESS_ACTIONS, QUALITY_ACTIONS,
};
use crate::solvers::{FinishSolver, UpperBoundSolver};

use std::time::Instant;
use std::vec::Vec;

const SEARCH_ACTIONS: ActionMask = PROGRESS_ACTIONS
    .union(QUALITY_ACTIONS)
    .union(MIXED_ACTIONS)
    .union(DURABILITY_ACTIONS);

pub struct MacroSolver {
    settings: Settings,
    finish_solver: FinishSolver,
    bound_solver: UpperBoundSolver,
}

impl MacroSolver {
    pub fn new(settings: Settings) -> MacroSolver {
        dbg!(std::mem::size_of::<SearchNode>());
        dbg!(std::mem::align_of::<SearchNode>());
        MacroSolver {
            settings,
            finish_solver: FinishSolver::new(settings),
            bound_solver: UpperBoundSolver::new(settings),
        }
    }

    pub fn solve(&mut self, state: State) -> Option<Vec<Action>> {
        match state {
            State::InProgress(state) => {
                if !self.finish_solver.can_finish(&state) {
                    return None;
                }
                match self.do_solve(state) {
                    Some(actions) => Some(actions),
                    None => Some(self.finish_solver.get_finish_sequence(state).unwrap()),
                }
            }
            _ => None,
        }
    }

    fn do_solve(&mut self, state: InProgress) -> Option<Vec<Action>> {
        let timer = Instant::now();

        let mut accepted_nodes: usize = 0;
        let mut finish_solver_rejected_node: usize = 0;
        let mut upper_bound_solver_rejected_nodes: usize = 0;

        let mut visited_states = FxHashSet::default();
        let mut search_queue = RadixHeapMap::new();

        let mut best_quality = Quality::new(0);
        let mut best_actions = None;

        visited_states.insert(state);
        search_queue.push(
            self.bound_solver.quality_upper_bound(state),
            SearchNode {
                state: state,
                actions: Vec::new(),
            },
        );

        while let Some((quality_bound, node)) = search_queue.pop() {
            accepted_nodes += 1;
            if best_quality == self.settings.max_quality || quality_bound <= best_quality {
                continue;
            }
            for action in SEARCH_ACTIONS
                .intersection(self.settings.allowed_actions)
                .actions_iter()
            {
                let state = node
                    .state
                    .use_action(action, Condition::Normal, &self.settings);
                if let State::InProgress(state) = state {
                    if !self.finish_solver.can_finish(&state) {
                        finish_solver_rejected_node += 1;
                        continue;
                    }
                    let quality_bound = self.bound_solver.quality_upper_bound(state);
                    if quality_bound <= best_quality {
                        upper_bound_solver_rejected_nodes += 1;
                        continue;
                    }
                    let quality = self.settings.max_quality.sub(state.missing_quality);
                    let actions: Vec<Action> =
                        node.actions.iter().chain(&[action]).copied().collect();
                    if quality > best_quality {
                        best_quality = quality;
                        let finish_actions = self.finish_solver.get_finish_sequence(state).unwrap();
                        best_actions =
                            Some(actions.iter().chain(&finish_actions).copied().collect());
                    }
                    if !visited_states.contains(&state) {
                        visited_states.insert(state);
                        search_queue.push(quality_bound, SearchNode { state, actions });
                    }
                }
            }
        }

        let seconds = timer.elapsed().as_secs_f32();
        dbg!(seconds);

        dbg!(
            accepted_nodes,
            finish_solver_rejected_node,
            upper_bound_solver_rejected_nodes
        );

        dbg!(best_quality, &best_actions);
        best_actions
    }
}

#[derive(Debug, Clone)]
struct SearchNode {
    pub state: InProgress,
    pub actions: Vec<Action>,
}
