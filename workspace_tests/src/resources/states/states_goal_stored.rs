use peace::resources::states::{StatesGoal, StatesGoalStored};

#[test]
fn from_states_goal() {
    let _states_goal_stored = StatesGoalStored::from(StatesGoal::new());
}
