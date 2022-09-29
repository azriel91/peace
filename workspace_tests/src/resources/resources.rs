use peace::resources::{
    resources::ts::{
        Cleaned, CleanedDry, Ensured, EnsuredDry, SetUp, WithStateDiffs, WithStates,
        WithStatesCurrentAndDesired, WithStatesDesired,
    },
    states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesCurrent, StatesDesired, StatesEnsured,
        StatesEnsuredDry,
    },
    Resources,
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
fn resources_ensured_dry_from_resources_with_state_diffs() {
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
    let resources_ensured_dry =
        Resources::<EnsuredDry>::from((resources_with_state_diffs, StatesEnsuredDry::new()));

    assert!(resources_ensured_dry.contains::<StatesCurrent>());
    assert!(resources_ensured_dry.contains::<StatesDesired>());
    assert!(resources_ensured_dry.contains::<StateDiffs>());
    assert!(resources_ensured_dry.contains::<StatesEnsuredDry>());
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

#[test]
fn resources_cleaned_dry_from_resources_with_states_current() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current =
        Resources::<WithStates>::from((resources_set_up, StatesCurrent::new()));
    let resources_cleaned_dry =
        Resources::<CleanedDry>::from((resources_with_states_current, StatesCleanedDry::new()));

    assert!(resources_cleaned_dry.contains::<StatesCurrent>());
    assert!(resources_cleaned_dry.contains::<StatesCleanedDry>());
}

#[test]
fn resources_cleaned_from_resources_with_states_current() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current =
        Resources::<WithStates>::from((resources_set_up, StatesCurrent::new()));
    let resources_cleaned =
        Resources::<Cleaned>::from((resources_with_states_current, StatesCleaned::new()));

    assert!(resources_cleaned.contains::<StatesCurrent>());
    assert!(resources_cleaned.contains::<StatesCleaned>());
}
