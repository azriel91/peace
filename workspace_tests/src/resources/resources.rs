use peace::resources::{
    resources_type_state::{SetUp, WithStateDiffs, WithStates, WithStatesDesired},
    Resources, StateDiffs, States, StatesDesired,
};

#[test]
fn resources_set_up_from_resources_empty() {
    let resources_empty = Resources::new();

    let resources_set_up = Resources::<SetUp>::from(resources_empty);

    // no default resources
    assert!(!resources_set_up.contains::<States>());
    assert!(!resources_set_up.contains::<StatesDesired>());
}

#[test]
fn resources_with_states_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states = Resources::<WithStates>::from((resources_set_up, States::new()));

    assert!(resources_with_states.contains::<States>());
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
    let resources_with_states_now_and_desired = Resources::<WithStateDiffs>::from((
        resources_set_up,
        States::new(),
        StatesDesired::new(),
        StateDiffs::new(),
    ));

    assert!(resources_with_states_now_and_desired.contains::<States>());
    assert!(resources_with_states_now_and_desired.contains::<StatesDesired>());
    assert!(resources_with_states_now_and_desired.contains::<StateDiffs>());
}
