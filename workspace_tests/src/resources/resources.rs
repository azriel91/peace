use peace::resources::{
    resources_type_state::{
        Ensured, SetUp, WithStateDiffs, WithStates, WithStatesCurrentAndDesired, WithStatesDesired,
    },
    Resources, StateDiffs, StatesCurrent, StatesDesired, StatesEnsured,
};

#[test]
fn resources_set_up_from_resources_empty() {
    let resources_empty = Resources::new();

    let resources_set_up = Resources::<SetUp>::from(resources_empty);

    // no default resources
    assert!(!resources_set_up.contains::<StatesCurrent>());
    assert!(!resources_set_up.contains::<StatesDesired>());
}

#[test]
fn resources_with_states_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states =
        Resources::<WithStates>::from((resources_set_up, StatesCurrent::new()));

    assert!(resources_with_states.contains::<StatesCurrent>());
}

#[test]
fn resources_with_states_desired_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_desired =
        Resources::<WithStatesDesired>::from((resources_set_up, StatesDesired::new()));

    assert!(resources_with_states_desired.contains::<StatesDesired>());
}

#[test]
fn resources_with_states_now_and_desired_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_now_and_desired = Resources::<WithStatesCurrentAndDesired>::from((
        resources_set_up,
        StatesCurrent::new(),
        StatesDesired::new(),
    ));

    assert!(resources_with_states_now_and_desired.contains::<StatesCurrent>());
    assert!(resources_with_states_now_and_desired.contains::<StatesDesired>());
}

#[test]
fn resources_with_state_diffs_from_resources_with_states_now_and_desired() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_now_and_desired = Resources::<WithStatesCurrentAndDesired>::from((
        resources_set_up,
        StatesCurrent::new(),
        StatesDesired::new(),
    ));
    let resources_with_state_diffs = Resources::<WithStateDiffs>::from((
        resources_with_states_now_and_desired,
        StateDiffs::new(),
    ));

    assert!(resources_with_state_diffs.contains::<StatesCurrent>());
    assert!(resources_with_state_diffs.contains::<StatesDesired>());
    assert!(resources_with_state_diffs.contains::<StateDiffs>());
}

#[test]
fn resources_ensured_from_resources_with_state_diffs() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_now_and_desired = Resources::<WithStatesCurrentAndDesired>::from((
        resources_set_up,
        StatesCurrent::new(),
        StatesDesired::new(),
    ));
    let resources_with_state_diffs = Resources::<WithStateDiffs>::from((
        resources_with_states_now_and_desired,
        StateDiffs::new(),
    ));
    let resources_ensured =
        Resources::<Ensured>::from((resources_with_state_diffs, StatesEnsured::new()));

    assert!(resources_ensured.contains::<StatesCurrent>());
    assert!(resources_ensured.contains::<StatesDesired>());
    assert!(resources_ensured.contains::<StateDiffs>());
    assert!(resources_ensured.contains::<StatesEnsured>());
}
