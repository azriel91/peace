use peace::resources::states::{StatesGoal, StatesGoalStored};

#[test]
fn from_states_goal_stored() {
    let _states_goal = StatesGoal::from(StatesGoalStored::new());
}
