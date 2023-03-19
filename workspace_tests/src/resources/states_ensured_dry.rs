use peace::resources::states::{StatesCurrent, StatesEnsuredDry};

#[test]
fn from_states_current() {
    let _states_ensured_dry = StatesEnsuredDry::from(StatesCurrent::new());
}
