use peace::resource_rt::states::{StatesCurrent, StatesEnsuredDry};

#[test]
fn from_states_current() {
    let _states_ensured_dry = StatesEnsuredDry::from(StatesCurrent::new());
}
