use peace::resources::states::{StatesCurrent, StatesCurrentStored};

#[test]
fn from_states_current_stored() {
    let _states_current = StatesCurrent::from(StatesCurrentStored::new());
}
