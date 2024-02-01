use peace::resources::states::{StatesCleanedDry, StatesCurrent};

#[test]
fn from_states_current() {
    let _states_cleaned_dry = StatesCleanedDry::from(StatesCurrent::new());
}
