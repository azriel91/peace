use peace::resources::{
    resources::ts::{SetUp, WithStates},
    states::{StatesCleaned, StatesCurrent},
    Resources,
};

#[test]
fn from_states_and_resources_with_states_current() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current =
        Resources::<WithStates>::from((resources_set_up, StatesCurrent::new()));

    let _states_cleaned =
        StatesCleaned::from((StatesCurrent::new(), &resources_with_states_current));
}
