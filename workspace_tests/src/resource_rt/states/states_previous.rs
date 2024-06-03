use peace::resource_rt::states::{StatesCurrent, StatesPrevious};

#[test]
fn from_states_current() {
    let _states_previous = StatesPrevious::from(StatesCurrent::new());
}
