use peace::resource_rt::states::{StatesCleanedDry, StatesCurrent};

#[test]
fn from_states_current() {
    let _states_cleaned_dry = StatesCleanedDry::from(StatesCurrent::new());
}
