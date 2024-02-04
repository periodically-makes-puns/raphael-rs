use crate::game::{state::InProgress, units::Quality, Action, Settings, State};
use crate::solvers::FinishSolver;

use std::time::Instant;
use std::vec::Vec;
use typed_arena::Arena;

use strum::IntoEnumIterator;

use super::*;

struct MacroResult {
    quality: Quality,
    actions: Vec<Action>,
}

pub struct MacroSolver<'a> {
    settings: Settings,
    search_queue: SearchQueue<'a>,
    explored_nodes: Arena<SearchNode<'a>>,
    finish_solver: FinishSolver,
}

impl<'a> MacroSolver<'a> {
    pub fn new(settings: Settings) -> MacroSolver<'a> {
        MacroSolver {
            settings: settings.clone(),
            search_queue: SearchQueue::new(settings.clone()),
            explored_nodes: Arena::new(),
            finish_solver: FinishSolver::new(settings),
        }
    }

    pub fn solve(&'a mut self, state: State) -> Option<Vec<Action>> {
        match state {
            State::InProgress(state) => {
                let result = self.do_solve(state);
                match result {
                    Some(result) => Some(result.actions),
                    None => None,
                }
            }
            _ => None,
        }
    }

    fn do_solve(&'a mut self, state: InProgress) -> Option<MacroResult> {
        let timer = Instant::now();

        self.search_queue.push(SearchNode { state, trace: None });

        let mut result = MacroResult {
            quality: Quality::from(0),
            actions: Vec::new(),
        };

        while let Some(current_node) = self.search_queue.pop() {
            let current_node: &SearchNode<'_> = self.explored_nodes.alloc(current_node);
            for sequence in ActionSequence::iter() {
                if sequence.should_use(&current_node.state) {
                    let use_action = sequence.apply(
                        State::InProgress(current_node.state.clone()),
                        &self.settings,
                    );
                    match use_action {
                        State::InProgress(state) => {
                            if self.finish_solver.can_finish(&state) {
                                if state.quality > result.quality {
                                    result = MacroResult {
                                        quality: state.quality,
                                        actions: SearchTrace {
                                            parent: current_node,
                                            action: sequence,
                                        }
                                        .actions(),
                                    };
                                    result.actions.append(
                                        &mut self
                                            .finish_solver
                                            .get_finish_sequence(&state)
                                            .unwrap(),
                                    );
                                    log::trace!(
                                        "result ({}): {:?}",
                                        result.quality,
                                        result.actions
                                    );
                                }
                                self.search_queue.push(SearchNode {
                                    state,
                                    trace: Some(SearchTrace {
                                        parent: current_node,
                                        action: sequence,
                                    }),
                                });
                            }
                        }
                        _ => (),
                    }
                }
            }
        }

        let time = timer.elapsed().as_secs_f32();
        let nodes = self.explored_nodes.len() as f32;
        log::debug!("Time elapsed: {}s", time);
        log::debug!(
            "Searched nodes: {:+.2e} ({:+.2e} nodes/s)",
            nodes,
            nodes / time
        );

        Some(result)
    }
}