use std::cmp::max;

use crate::{effects::SingleUse, Action, ComboAction, Condition, Effects, Settings};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PrevActionDelta {
    pub to_excellent: u16,
    pub to_poor: u16,
}

impl Default for PrevActionDelta {
    fn default() -> Self {
        PrevActionDelta {
            to_excellent: 0, to_poor: 0
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SimulationState {
    pub cp: i16,
    pub durability: i8,
    pub missing_progress: u16,
    pub missing_quality: [u16; 3],
    pub prev_deltas: [PrevActionDelta; 2],
    pub effects: Effects,
    pub combo: Option<ComboAction>,
}

impl SimulationState {
    pub fn new(settings: &Settings) -> Self {
        Self {
            cp: settings.max_cp,
            durability: settings.max_durability,
            missing_progress: settings.max_progress,
            missing_quality: [settings
                .max_quality
                .saturating_sub(settings.initial_quality), 0, 0],
            prev_deltas: [PrevActionDelta::default(); 2],
            effects: Default::default(),
            combo: Some(ComboAction::SynthesisBegin),
        }
    }

    pub fn from_macro(settings: &Settings, actions: &[Action]) -> Result<Self, &'static str> {
        let mut state = Self::new(settings);
        for action in actions {
            let in_progress: InProgress = state.try_into()?;
            state = in_progress.use_action(*action, Condition::Normal, settings)?;
        }
        Ok(state)
    }

    pub fn from_macro_continue_on_error(
        settings: &Settings,
        actions: &[Action],
    ) -> (Self, Vec<Result<(), &'static str>>) {
        let mut state = Self::new(settings);
        let mut errors = Vec::new();
        for action in actions {
            state = match InProgress::try_from(state) {
                Ok(in_progress) => {
                    match in_progress.use_action(*action, Condition::Normal, settings) {
                        Ok(new_state) => {
                            errors.push(Ok(()));
                            new_state
                        }
                        Err(err) => {
                            errors.push(Err(err));
                            state
                        }
                    }
                }
                Err(err) => {
                    errors.push(Err(err));
                    state
                }
            };
        }
        (state, errors)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InProgress {
    state: SimulationState,
}

impl TryFrom<SimulationState> for InProgress {
    type Error = &'static str;

    fn try_from(value: SimulationState) -> Result<Self, Self::Error> {
        if value.missing_progress == 0 {
            return Err("Progress already at 100%");
        }
        if value.durability <= 0 {
            return Err("No remaining durability");
        }
        Ok(Self { state: value })
    }
}

impl InProgress {
    pub fn new(settings: &Settings) -> Self {
        Self {
            state: SimulationState::new(settings),
        }
    }

    pub fn raw_state(&self) -> &SimulationState {
        &self.state
    }

    pub fn can_use_action(
        &self,
        action: Action,
        condition: Condition,
        settings: &Settings,
    ) -> Result<(), &'static str> {
        if !settings.allowed_actions.has(action) {
            return Err("Action not enabled");
        }
        if action.cp_cost(&self.state.effects, condition) > self.state.cp {
            return Err("Not enough CP");
        }
        if !action.combo_fulfilled(self.state.combo) {
            return Err("Combo requirement not fulfilled");
        }
        match action {
            Action::ByregotsBlessing if self.state.effects.inner_quiet() == 0 => {
                Err("Need Inner Quiet to use Byregot's Blessing")
            }
            Action::PrudentSynthesis | Action::PrudentTouch
                if self.state.effects.waste_not() != 0 =>
            {
                Err("Action cannot be used during Waste Not")
            }
            Action::IntensiveSynthesis | Action::PreciseTouch | Action::TricksOfTheTrade
                if condition != Condition::Good && condition != Condition::Excellent =>
            {
                Err("Requires condition to be Good or Excellent")
            }
            Action::Groundwork
                if self.state.durability
                    < action.durability_cost(&self.state.effects, condition) =>
            {
                Err("Not enough durability")
            }
            Action::TrainedFinesse if self.state.effects.inner_quiet() < 10 => {
                Err("Requires 10 Inner Quiet")
            }
            Action::TrainedPerfection
                if !matches!(
                    self.state.effects.trained_perfection(),
                    SingleUse::Available
                ) =>
            {
                Err("Action can only be used once per synthesis")
            }
            _ => Ok(()),
        }
    }

    pub fn use_action(
        self,
        action: Action,
        condition: Condition,
        settings: &Settings,
    ) -> Result<SimulationState, &'static str> {
        self.can_use_action(action, condition, settings)?;
        let mut state = self.state;

        let cp_cost = action.cp_cost(&state.effects, condition);
        let durability_cost = action.durability_cost(&state.effects, condition);
        let progress_increase = action.progress_increase(settings, &state.effects, condition);
        let quality_increase = action.quality_increase(settings, &state.effects, condition);

        state.combo = action.to_combo();
        state.cp -= cp_cost;
        state.durability -= durability_cost;

        if action.base_durability_cost() != 0
            && state.effects.trained_perfection() == SingleUse::Active
        {
            state.effects.set_trained_perfection(SingleUse::Unavailable);
        }

        // reset muscle memory if progress increased
        if progress_increase != 0 {
            state.missing_progress = state.missing_progress.saturating_sub(progress_increase);
            state.effects.set_muscle_memory(0);
        }

        if settings.adversarial {
            let saved = state.missing_quality[2];
            state.missing_quality[2] = state.missing_quality[1];
            state.missing_quality[1] = state.missing_quality[0];
            state.missing_quality[0] = max(
                state.missing_quality[0], 
                saved.saturating_sub(state.prev_deltas[0].to_poor).saturating_sub(state.prev_deltas[1].to_excellent));
            state.prev_deltas[1] = state.prev_deltas[0];
            state.prev_deltas[0].to_excellent = action.quality_increase(settings, &state.effects, Condition::Excellent);
            state.prev_deltas[0].to_poor = action.quality_increase(settings, &state.effects, Condition::Poor);
        }

        // reset great strides and increase inner quiet if quality increased
        if quality_increase != 0 {
            state.missing_quality[0] = state.missing_quality[0].saturating_sub(quality_increase);
            state.effects.set_great_strides(0);
            if settings.job_level >= 11 {
                let inner_quiet_bonus = match action {
                    Action::Reflect => 2,
                    Action::PreciseTouch => 2,
                    Action::PreparatoryTouch => 2,
                    Action::ComboRefinedTouch => 2,
                    _ => 1,
                };
                state.effects.set_inner_quiet(std::cmp::min(
                    10,
                    state.effects.inner_quiet() + inner_quiet_bonus,
                ));
            }
        }

        if state.missing_progress == 0 || state.durability <= 0 {
            return Ok(state);
        }

        // remove manipulation before it is triggered
        if action == Action::Manipulation {
            state.effects.set_manipulation(0);
        }

        if state.effects.manipulation() > 0 {
            state.durability = std::cmp::min(state.durability + 5, settings.max_durability);
        }
        state.effects.tick_down();

        // trigger special action effects
        let duration_bonus = if condition == Condition::Pliant { 2 } else { 0 };
        match action {
            Action::MuscleMemory => state.effects.set_muscle_memory(5 + duration_bonus),
            Action::GreatStrides => state.effects.set_great_strides(3 + duration_bonus),
            Action::Veneration => state.effects.set_veneration(4 + duration_bonus),
            Action::Innovation => state.effects.set_innovation(4 + duration_bonus),
            Action::WasteNot => state.effects.set_waste_not(4 + duration_bonus),
            Action::WasteNot2 => state.effects.set_waste_not(8 + duration_bonus),
            Action::Manipulation => state.effects.set_manipulation(8 + duration_bonus),
            Action::MasterMend => {
                state.durability = std::cmp::min(settings.max_durability, state.durability + 30)
            }
            Action::ByregotsBlessing => state.effects.set_inner_quiet(0),
            Action::TricksOfTheTrade => state.cp = std::cmp::min(settings.max_cp, state.cp + 20),
            Action::ImmaculateMend => state.durability = settings.max_durability,
            Action::TrainedPerfection => state.effects.set_trained_perfection(SingleUse::Active),
            _ => (),
        }

        Ok(state)
    }
}
