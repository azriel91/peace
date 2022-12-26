use std::any::{Any, TypeId};

use peace::resources::{
    resources::ts::{
        Cleaned, CleanedDry, Empty, Ensured, EnsuredDry, SetUp, WithStatesCurrent,
        WithStatesCurrentAndDesired, WithStatesCurrentDiffs, WithStatesDesired, WithStatesSaved,
        WithStatesSavedAndDesired, WithStatesSavedDiffs,
    },
    states::{
        StateDiffs, StatesCleaned, StatesCleanedDry, StatesCurrent, StatesDesired, StatesEnsured,
        StatesEnsuredDry, StatesSaved,
    },
    Resources,
};

mod ts;

#[test]
fn debug() {
    let mut resources = Resources::new();
    resources.insert(1u32);

    assert_eq!(
        r#"Resources { inner: {u32: 1}, marker: PhantomData<peace_resources::resources::ts::Empty> }"#,
        format!("{resources:?}")
    );
}

#[test]
fn defaults_to_resources_empty() {
    let resources_default = Resources::default();

    assert_eq!(
        TypeId::of::<Resources::<Empty>>(),
        resources_default.type_id()
    );
}

#[test]
fn resources_set_up_from_resources_empty() {
    let resources_empty = Resources::new();

    let resources_set_up = Resources::<SetUp>::from(resources_empty);

    // no default resources
    assert!(!resources_set_up.contains::<StatesCurrent>());
    assert!(!resources_set_up.contains::<StatesDesired>());
}

#[test]
fn resources_with_states_saved_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_saved =
        Resources::<WithStatesSaved>::from((resources_set_up, StatesSaved::new()));

    assert!(resources_with_states_saved.contains::<StatesSaved>());
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
fn resources_with_states_saved_and_desired_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_saved_and_desired = Resources::<WithStatesSavedAndDesired>::from((
        resources_set_up,
        StatesSaved::new(),
        StatesDesired::new(),
    ));

    assert!(resources_with_states_saved_and_desired.contains::<StatesSaved>());
    assert!(resources_with_states_saved_and_desired.contains::<StatesDesired>());
}

#[test]
fn resources_with_state_saved_diffs_from_resources_with_states_saved_and_desired() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_saved_and_desired = Resources::<WithStatesSavedAndDesired>::from((
        resources_set_up,
        StatesSaved::new(),
        StatesDesired::new(),
    ));
    let resources_with_state_saved_diffs = Resources::<WithStatesSavedDiffs>::from((
        resources_with_states_saved_and_desired,
        StateDiffs::new(),
    ));

    assert!(resources_with_state_saved_diffs.contains::<StatesSaved>());
    assert!(resources_with_state_saved_diffs.contains::<StatesDesired>());
    assert!(resources_with_state_saved_diffs.contains::<StateDiffs>());
}

#[test]
fn resources_with_state_current_diffs_from_resources_with_states_current_and_desired() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current_and_desired = Resources::<WithStatesCurrentAndDesired>::from(
        (resources_set_up, StatesCurrent::new(), StatesDesired::new()),
    );
    let resources_with_state_current_diffs = Resources::<WithStatesCurrentDiffs>::from((
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
    let resources_with_state_current_diffs = Resources::<WithStatesCurrentDiffs>::from((
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
    let resources_with_state_current_diffs = Resources::<WithStatesCurrentDiffs>::from((
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
