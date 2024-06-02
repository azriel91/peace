use peace::resource_rt::states::{StatesCurrent, StatesEnsured};

#[test]
fn from_states_current() {
    let _states_ensured = StatesEnsured::from(StatesCurrent::new());
}
