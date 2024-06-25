use simulator::{state::InProgress, Action, ActionMask, Condition, Settings, SimulationState};
use solvers::MacroSolver;

fn solve(settings: &Settings) -> Option<Vec<Action>> {
    MacroSolver::new(settings.clone()).solve(InProgress::new(settings))
}

fn get_quality(settings: &Settings, actions: &[Action]) -> u32 {
    let mut state = SimulationState::new(&settings);
    for action in actions {
        state = InProgress::try_from(state)
            .unwrap()
            .use_action(action.clone(), Condition::Normal, &settings)
            .unwrap();
    }
    assert_eq!(state.missing_progress, 0);
    settings.max_quality - state.missing_quality
}

#[test]
fn test_random_0f93c79f() {
    let settings = Settings {
        max_cp: 370,
        max_durability: 60,
        max_progress: 2000,
        max_quality: 40000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 1682);
}

#[test]
fn test_random_1e281667() {
    let settings = Settings {
        max_cp: 553,
        max_durability: 70,
        max_progress: 2400,
        max_quality: 20000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 3352);
}

#[test]
fn test_random_d0bf2aef() {
    let settings = Settings {
        max_cp: 612,
        max_durability: 60,
        max_progress: 2560,
        max_quality: 40000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 3407);
}

#[test]
fn test_unsolvable() {
    let settings = Settings {
        max_cp: 100,
        max_durability: 60,
        max_progress: 4000,
        max_quality: 1000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings);
    assert_eq!(actions, None);
}

#[test]
fn test_max_quality() {
    let settings = Settings {
        max_cp: 400,
        max_durability: 60,
        max_progress: 2000,
        max_quality: 1000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 1000);
}

#[test]
fn test_zero_quality() {
    let settings = Settings {
        max_cp: 80,
        max_durability: 60,
        max_progress: 1920,
        max_quality: 1000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 0);
}

#[test]
fn test_random_e413e05d() {
    let settings = Settings {
        max_cp: 450,
        max_durability: 80,
        max_progress: 2800,
        max_quality: 40000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 2018);
}

#[test]
fn test_random_bb38a037() {
    let settings = Settings {
        max_cp: 540,
        max_durability: 70,
        max_progress: 2700,
        max_quality: 40000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 2752);
}

#[test]
fn test_random_a300ca2b() {
    let settings = Settings {
        max_cp: 700,
        max_durability: 70,
        max_progress: 2500,
        max_quality: 40000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 4683);
}

#[test]
fn test_random_0f9d7781() {
    let settings = Settings {
        max_cp: 701,
        max_durability: 60,
        max_progress: 3950,
        max_quality: 6950,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 2839);
}

#[test]
fn test_random_e451d981() {
    let settings = Settings {
        max_cp: 606,
        max_durability: 80,
        max_progress: 1200,
        max_quality: 20000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 5212);
}

#[test]
fn test_random_6799bb1d() {
    let settings = Settings {
        max_cp: 501,
        max_durability: 70,
        max_progress: 1950,
        max_quality: 20000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 3261);
}

#[test]
fn test_random_940b4755() {
    let settings = Settings {
        max_cp: 640,
        max_durability: 70,
        max_progress: 2170,
        max_quality: 20000,
        base_progress: 100,
        base_quality: 100,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 4403);
}

#[test]
fn test_rinascita_3700_3280() {
    let settings = Settings {
        max_cp: 680,
        max_durability: 70,
        max_progress: 5060,
        max_quality: 12628,
        base_progress: 229,
        base_quality: 224,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 10514);
}

#[test]
fn test_pactmaker_3240_3130() {
    let settings = Settings {
        max_cp: 600,
        max_durability: 70,
        max_progress: 4300,
        max_quality: 12800,
        base_progress: 200,
        base_quality: 215,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 8758);
}

#[test]
fn test_diadochos_4021_3660() {
    let settings = Settings {
        max_cp: 640,
        max_durability: 70,
        max_progress: 6600,
        max_quality: 14040,
        base_progress: 249,
        base_quality: 247,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 9688);
}

#[test]
fn test_indagator_3858_4057() {
    let settings = Settings {
        max_cp: 687,
        max_durability: 70,
        max_progress: 5720,
        max_quality: 12900,
        base_progress: 239,
        base_quality: 271,
        initial_quality: 0,
        job_level: 90,
        allowed_actions: ActionMask::from_level(90, true),
    };
    let actions = solve(&settings).unwrap();
    assert_eq!(get_quality(&settings, &actions), 12793);
}