use crate::{
    config::Settings,
    game::{
        actions::Action,
        effects::Effects,
        state::{InProgress, State},
    },
    solvers::util::action_sequence::ActionSequence,
};
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ReducedEffects {
    pub waste_not: i8,
    pub veneration: i8,
    pub manipulation: i8,
}

impl ReducedEffects {
    pub fn from_effects(effects: &Effects) -> ReducedEffects {
        ReducedEffects {
            waste_not: effects.waste_not,
            veneration: effects.veneration,
            manipulation: effects.manipulation,
        }
    }

    pub fn to_effects(&self) -> Effects {
        Effects {
            inner_quiet: 0,
            waste_not: self.waste_not,
            innovation: 0,
            veneration: self.veneration,
            great_strides: 0,
            muscle_memory: 0,
            manipulation: self.manipulation,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ReducedState {
    durability: i32,
    progress: i32,
    effects: ReducedEffects,
}

impl ReducedState {
    pub const MAX_CP: i32 = 100000;

    pub fn from_state(state: &InProgress) -> ReducedState {
        ReducedState {
            durability: state.durability,
            progress: state.progress,
            effects: ReducedEffects::from_effects(&state.effects),
        }
    }

    pub fn to_state(&self) -> InProgress {
        InProgress {
            last_action: Some(Action::TricksOfTheTrade),
            cp: ReducedState::MAX_CP,
            durability: self.durability,
            progress: self.progress,
            quality: 0,
            effects: self.effects.to_effects(),
        }
    }
}

#[derive(Debug)]
pub struct FinishSolver {
    settings: Settings,
    cp_to_finish: HashMap<ReducedState, i32>,
}

impl FinishSolver {
    pub fn new(settings: Settings) -> FinishSolver {
        FinishSolver {
            settings,
            cp_to_finish: HashMap::new(),
        }
    }

    pub fn get_finish_sequence(&self, state: &InProgress) -> Option<Vec<Action>> {
        let reduced_state = ReducedState::from_state(&state);
        match self.cp_to_finish.get(&reduced_state) {
            Some(cp) => {
                if state.cp >= *cp {
                    let mut result: Vec<Action> = Vec::new();
                    self.do_trace(&mut result, reduced_state, *cp);
                    Some(result)
                } else {
                    None
                }
            }
            None => None,
        }
    }

    fn do_trace(&self, result: &mut Vec<Action>, state: ReducedState, cp_budget: i32) -> () {
        for sequence in ACTION_SEQUENCES {
            let target_cp = cp_budget - sequence.base_cp_cost();
            if target_cp >= 0 && self.should_use(&state, sequence) {
                match sequence.apply(State::InProgress(state.to_state()), &self.settings) {
                    State::InProgress(new_state) => {
                        let new_state = ReducedState::from_state(&new_state);
                        let new_state_cost = *self.cp_to_finish.get(&new_state).unwrap();
                        if new_state_cost == target_cp {
                            result.extend_from_slice(sequence.actions());
                            self.do_trace(result, new_state, target_cp);
                            return;
                        }
                    }
                    State::Completed(_) => {
                        result.extend_from_slice(sequence.actions());
                        return;
                    }
                    _ => (),
                }
            }
        }
        panic!()
    }

    pub fn can_finish(&mut self, state: &InProgress) -> bool {
        state.cp >= self.do_solve(ReducedState::from_state(&state))
    }

    fn do_solve(&mut self, state: ReducedState) -> i32 {
        match self.cp_to_finish.get(&state) {
            Some(cost) => *cost,
            None => {
                let mut result: i32 = ReducedState::MAX_CP;
                for sequence in ACTION_SEQUENCES {
                    if self.should_use(&state, sequence) {
                        match sequence.apply(State::InProgress(state.to_state()), &self.settings) {
                            State::InProgress(new_state) => {
                                let new_state_cost =
                                    self.do_solve(ReducedState::from_state(&new_state));
                                result =
                                    std::cmp::min(result, new_state_cost + sequence.base_cp_cost());
                            }
                            State::Completed(_) => {
                                result = std::cmp::min(result, sequence.base_cp_cost());
                            }
                            State::Invalid => (),
                            State::Failed => (),
                        }
                    }
                }
                self.cp_to_finish.insert(state, result);
                result
            }
        }
    }

    fn should_use(&self, state: &ReducedState, sequence: ActionSequence) -> bool {
        let manipulation_capped =
            state.effects.manipulation != 0 && state.durability == self.settings.max_durability;
        match sequence {
            ActionSequence::MasterMend => state.durability + 30 <= self.settings.max_durability,
            ActionSequence::BasicSynthesis => true,
            ActionSequence::CarefulSynthesis => true,
            ActionSequence::Groundwork => true,
            ActionSequence::FocusedSynthesisCombo => {
                !manipulation_capped
                    && state.effects.waste_not == 0
                    && (state.effects.veneration >= 2 || state.effects.veneration == 0)
            }
            ActionSequence::Manipulation => state.effects.manipulation == 0,
            ActionSequence::WasteNot | ActionSequence::WasteNot2 => {
                !manipulation_capped && state.effects.waste_not == 0
            }
            ActionSequence::Veneration => !manipulation_capped && state.effects.veneration == 0,
            _ => false,
        }
    }
}

impl Drop for FinishSolver {
    fn drop(&mut self) {
        log::debug!(
            "FinishSolver: {:+.2e} states",
            self.cp_to_finish.len() as f32
        );
    }
}

const ACTION_SEQUENCES: [ActionSequence; 9] = [
    ActionSequence::BasicSynthesis,
    ActionSequence::MasterMend,
    ActionSequence::CarefulSynthesis,
    ActionSequence::Groundwork,
    ActionSequence::FocusedSynthesisCombo,
    ActionSequence::Manipulation,
    ActionSequence::WasteNot,
    ActionSequence::WasteNot2,
    ActionSequence::Veneration,
];
