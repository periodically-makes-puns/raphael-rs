use std::cmp::min;

use simulator::{state::InProgress, Action, ActionMask, Condition, Settings, SimulationState};

const SETTINGS: Settings = Settings {
    max_cp: 250,
    max_durability: 60,
    max_progress: 2000,
    max_quality: 40000,
    base_progress: 100,
    base_quality: 100,
    initial_quality: 0,
    job_level: 100,
    allowed_actions: ActionMask::all(),
    adversarial: false,
};

#[derive(Debug)]
struct SavedQuality {
    pub excellent: u16,
    pub normal: u16,
    pub poor: u16,
}

fn get_adversarial_quality(settings: &Settings, actions: &[Action]) -> Option<u16> {
    let settings = &Settings {
        adversarial: false,
        ..*settings
    };
    let state = SimulationState::new(settings);
    let saved_qual: Vec<SavedQuality> = actions.iter().scan(state, |st, action| {
        let in_progress = InProgress::try_from(*st).ok()?;
        let prev_quality = st.get_missing_quality();
        let next = in_progress.use_action(*action, Condition::Normal, settings).ok()?;
        let ret = SavedQuality {
            excellent: prev_quality - (in_progress.use_action(*action, Condition::Excellent, settings).ok()?.get_missing_quality()),
            normal: prev_quality - (next.get_missing_quality()),
            poor: prev_quality - (in_progress.use_action(*action, Condition::Poor, settings).ok()?.get_missing_quality()),
        };
        st.clone_from(&next);
        Some(ret)
    }).collect();
    let mut res: Vec<(u16, u16)> = vec![(saved_qual[0].normal, saved_qual[0].normal)];
    for i in 1..saved_qual.len() {
        res.push((
            min(res[i-1].0, res[i-1].1) + saved_qual[i].normal, 
            min(if i >= 2 {res[i-2].0 + saved_qual[i-1].excellent + saved_qual[i].poor} else {u16::MAX}, 
                min(res[i-1].0, res[i-1].1) + saved_qual[i].normal
            )
        ));
    }
    dbg!(saved_qual);
    dbg!(res.clone());
    Some(res.last()?.1)
}

#[test]
fn test_adversarial_calculation() {
    let settings = Settings {
        adversarial: true,
        ..SETTINGS
    };
    let actions = &[Action::Observe, Action::Observe, Action::PreparatoryTouch, Action::BasicSynthesis];
    let state =
    SimulationState::from_macro(&settings, actions);
    if let Ok(state) = state {
        let expected = get_adversarial_quality(&settings, actions).unwrap();
        assert_eq!(settings.max_quality - state.get_missing_quality(), expected);
    } else {
        panic!("Unexpected err: {}", state.err().unwrap());
    }
}

#[test]
fn test_flipping() {
    let settings = Settings {
        adversarial: true,
        ..SETTINGS
    };
    let actions = &[
        Action::MuscleMemory, 
        Action::GreatStrides, 
        Action::BasicTouch, 
        Action::GreatStrides, 
        Action::BasicTouch, 
        Action::GreatStrides, 
        Action::BasicTouch
    ];
    let state =
    SimulationState::from_macro(&settings, actions);
    if let Ok(state) = state {
        let expected = get_adversarial_quality(&settings, actions).unwrap();
        assert_eq!(settings.max_quality - state.get_missing_quality(), expected);
    } else {
        panic!("Unexpected err: {}", state.err().unwrap());
    }
}

#[test]
fn test_double_status_drops_unreliable() {
    let settings = Settings {
        adversarial: true,
        ..SETTINGS
    };
    let actions = &[
        Action::MuscleMemory, 
        Action::GreatStrides, 
        Action::BasicTouch, 
        Action::Innovation,
        Action::GreatStrides, 
        Action::BasicTouch, 
        Action::GreatStrides, 
        Action::BasicTouch
    ];
    let state =
    SimulationState::from_macro(&settings, actions);
    if let Ok(state) = state {
        let expected = get_adversarial_quality(&settings, actions).unwrap();
        assert_eq!(settings.max_quality - state.get_missing_quality(), expected);
    } else {
        panic!("Unexpected err: {}", state.err().unwrap());
    }
}

#[test]
fn test_two_actions_drop_unreliable() {
    let settings = Settings {
        adversarial: true,
        ..SETTINGS
    };
    let actions = &[
        Action::MuscleMemory, 
        Action::GreatStrides, 
        Action::BasicTouch, 
        Action::StandardTouch,
        Action::GreatStrides, 
        Action::BasicTouch, 
        Action::GreatStrides, 
        Action::BasicTouch
    ];
    let state =
    SimulationState::from_macro(&settings, actions);
    if let Ok(state) = state {
        let expected = get_adversarial_quality(&settings, actions).unwrap();
        assert_eq!(settings.max_quality - state.get_missing_quality(), expected);
    } else {
        panic!("Unexpected err: {}", state.err().unwrap());
    }
}

#[test]
fn test_unreliable_dp() {
    let settings = Settings {
        adversarial: true,
        max_durability: 80,
        max_cp: 1000,
        ..SETTINGS
    };
    let actions = &[
        Action::MuscleMemory, 
        Action::GreatStrides, 
        Action::PreparatoryTouch,
        Action::Innovation,
        Action::BasicTouch,
        Action::Observe,
        Action::AdvancedTouch,
        Action::GreatStrides,
        Action::PreparatoryTouch
    ];
    let state =
    SimulationState::from_macro(&settings, actions);
    if let Ok(state) = state {
        let expected = get_adversarial_quality(&settings, actions).unwrap();
        assert_eq!(settings.max_quality - state.get_missing_quality(), expected);
    } else {
        panic!("Unexpected err: {}", state.err().unwrap());
    }
}