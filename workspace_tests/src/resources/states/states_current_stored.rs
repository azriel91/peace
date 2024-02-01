use peace::resources::states::{StatesCurrent, StatesCurrentStored};

#[test]
fn from_states_current() {
    let _states_current_stored = StatesCurrentStored::<ItemIdT>::from(StatesCurrent::new());
}
