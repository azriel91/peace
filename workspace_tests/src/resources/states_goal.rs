use peace::resources::states::{StatesCurrent, StatesCurrentStored};

#[test]
fn from_states_goal_stored() {
    let _states_goal = StatesCurrent::from(StatesCurrentStored::new());
}
