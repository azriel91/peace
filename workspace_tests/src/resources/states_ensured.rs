use peace::resources::{
    resources_type_state::{SetUp, WithStateDiffs, WithStatesCurrentAndDesired},
    states::{StateDiffs, StatesCurrent, StatesDesired, StatesEnsured},
    Resources,
};

#[test]
fn from_states_and_resources_with_state_diffs() {
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

    let _states_ensured = StatesEnsured::from((StatesCurrent::new(), &resources_with_state_diffs));
}
