use peace::resources::{
    resources::ts::{SetUp, WithStateCurrentDiffs, WithStatesCurrentAndDesired},
    states::{StateDiffs, StatesCurrent, StatesDesired, StatesEnsuredDry},
    Resources,
};

#[test]
fn from_states_and_resources_with_state_current_diffs() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current_and_desired = Resources::<WithStatesCurrentAndDesired>::from(
        (resources_set_up, StatesCurrent::new(), StatesDesired::new()),
    );
    let resources_with_state_current_diffs = Resources::<WithStateCurrentDiffs>::from((
        resources_with_states_current_and_desired,
        StateDiffs::new(),
    ));

    let _states_ensured_dry =
        StatesEnsuredDry::from((StatesCurrent::new(), &resources_with_state_current_diffs));
}
