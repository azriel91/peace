use peace::resources::{
    resources_type_state::{SetUp, WithStates},
    Resources, States, StatesDesiredRw, StatesRw,
};

#[test]
fn resources_set_up_from_resources_empty() {
    let resources_empty = Resources::new();

    assert!(resources_empty.contains::<StatesRw>());
    assert!(resources_empty.contains::<StatesDesiredRw>());

    let resources_set_up = Resources::<SetUp>::from(resources_empty);

    assert!(resources_set_up.contains::<StatesRw>());
    assert!(resources_set_up.contains::<StatesDesiredRw>());
}

#[test]
fn resources_with_states_from_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states = Resources::<WithStates>::from(resources_set_up);

    assert!(!resources_with_states.contains::<StatesRw>());
    assert!(resources_with_states.contains::<States>());

    // TODO: similar for `StatesDesiredRw`
}
