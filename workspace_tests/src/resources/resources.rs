use peace::resources::{
    resources::ts::{
        Cleaned, CleanedDry, Ensured, EnsuredDry, SetUp, WithStateCurrentDiffs,
        WithStatePreviousDiffs, WithStatesCurrent, WithStatesCurrentAndDesired, WithStatesDesired,
        WithStatesPrevious, WithStatesPreviousAndDesired,
    },
    states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesCurrent, StatesDesired, StatesEnsured,
        StatesEnsuredDry, StatesPrevious,
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
fn resources_with_states_previous_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_previous =
        Resources::<WithStatesPrevious>::from((resources_set_up, StatesPrevious::new()));

    assert!(resources_with_states_previous.contains::<StatesPrevious>());
}

#[test]
fn resources_with_states_current_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current =
        Resources::<WithStatesCurrent>::from((resources_set_up, StatesCurrent::new()));

    assert!(resources_with_states_current.contains::<StatesCurrent>());
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
fn resources_with_states_previous_and_desired_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_previous_and_desired =
        Resources::<WithStatesPreviousAndDesired>::from((
            resources_set_up,
            StatesPrevious::new(),
            StatesDesired::new(),
        ));

    assert!(resources_with_states_previous_and_desired.contains::<StatesPrevious>());
    assert!(resources_with_states_previous_and_desired.contains::<StatesDesired>());
}

#[test]
fn resources_with_state_previous_diffs_from_resources_with_states_previous_and_desired() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_previous_and_desired =
        Resources::<WithStatesPreviousAndDesired>::from((
            resources_set_up,
            StatesPrevious::new(),
            StatesDesired::new(),
        ));
    let resources_with_state_previous_diffs = Resources::<WithStatePreviousDiffs>::from((
        resources_with_states_previous_and_desired,
        StateDiffs::new(),
    ));

    assert!(resources_with_state_previous_diffs.contains::<StatesPrevious>());
    assert!(resources_with_state_previous_diffs.contains::<StatesDesired>());
    assert!(resources_with_state_previous_diffs.contains::<StateDiffs>());
}

#[test]
fn resources_with_state_current_diffs_from_resources_with_states_current_and_desired() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current_and_desired = Resources::<WithStatesCurrentAndDesired>::from(
        (resources_set_up, StatesCurrent::new(), StatesDesired::new()),
    );
    let resources_with_state_current_diffs = Resources::<WithStateCurrentDiffs>::from((
        resources_with_states_current_and_desired,
        StateDiffs::new(),
    ));

    assert!(resources_with_state_current_diffs.contains::<StatesCurrent>());
    assert!(resources_with_state_current_diffs.contains::<StatesDesired>());
    assert!(resources_with_state_current_diffs.contains::<StateDiffs>());
}

#[test]
fn resources_ensured_dry_from_resources_with_state_current_diffs() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current_and_desired = Resources::<WithStatesCurrentAndDesired>::from(
        (resources_set_up, StatesCurrent::new(), StatesDesired::new()),
    );
    let resources_with_state_current_diffs = Resources::<WithStateCurrentDiffs>::from((
        resources_with_states_current_and_desired,
        StateDiffs::new(),
    ));
    let resources_ensured_dry = Resources::<EnsuredDry>::from((
        resources_with_state_current_diffs,
        StatesEnsuredDry::new(),
    ));

    assert!(resources_ensured_dry.contains::<StatesCurrent>());
    assert!(resources_ensured_dry.contains::<StatesDesired>());
    assert!(resources_ensured_dry.contains::<StateDiffs>());
    assert!(resources_ensured_dry.contains::<StatesEnsuredDry>());
}

#[test]
fn resources_ensured_from_resources_with_state_current_diffs() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current_and_desired = Resources::<WithStatesCurrentAndDesired>::from(
        (resources_set_up, StatesCurrent::new(), StatesDesired::new()),
    );
    let resources_with_state_current_diffs = Resources::<WithStateCurrentDiffs>::from((
        resources_with_states_current_and_desired,
        StateDiffs::new(),
    ));
    let resources_ensured =
        Resources::<Ensured>::from((resources_with_state_current_diffs, StatesEnsured::new()));

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
        Resources::<WithStatesCurrent>::from((resources_set_up, StatesCurrent::new()));
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
        Resources::<WithStatesCurrent>::from((resources_set_up, StatesCurrent::new()));
    let resources_cleaned =
        Resources::<Cleaned>::from((resources_with_states_current, StatesCleaned::new()));

    assert!(resources_cleaned.contains::<StatesCurrent>());
    assert!(resources_cleaned.contains::<StatesCleaned>());
}
