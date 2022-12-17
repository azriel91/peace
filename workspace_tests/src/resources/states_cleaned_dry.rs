use peace::resources::{
    resources::ts::{SetUp, WithStatesCurrent},
    states::{StatesCleanedDry, StatesCurrent},
    Resources,
};

#[test]
fn from_states_and_resources_with_states_current() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);
    let resources_with_states_current =
        Resources::<WithStatesCurrent>::from((resources_set_up, StatesCurrent::new()));

    let _states_cleaned_dry =
        StatesCleanedDry::from((StatesCurrent::new(), &resources_with_states_current));
}
