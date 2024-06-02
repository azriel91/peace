use peace::resource_rt::states::{StatesCurrent, StatesCurrentStored};

#[test]
fn from_states_current() {
    let _states_current_stored = StatesCurrentStored::from(StatesCurrent::new());
}
