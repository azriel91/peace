use peace::resources::{
    resources::ts::SetUp,
    states::{StatesCurrent, StatesEnsuredDry},
    Resources,
};

#[test]
fn from_states_and_resources_set_up() {
    let resources_empty = Resources::new();
    let resources_set_up = Resources::<SetUp>::from(resources_empty);

    let _states_ensured_dry = StatesEnsuredDry::from((StatesCurrent::new(), &resources_set_up));
}
