use peace::resources::states::{StatesCleaned, StatesCurrent};

#[test]
fn from_states_current() {
    let _states_cleaned = StatesCleaned::from(StatesCurrent::new());
}
