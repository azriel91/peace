use std::marker::PhantomData;

use crate::states::{
    ts::{Goal, GoalStored},
    States,
};

/// Stored goal `State`s for all `Step`s.
///
/// These are the states that each step would be in, if `Step::apply` were to be
/// run with `state_goal` as the target state.
///
/// This is loaded into [`Resources`] at the beginning of any command execution,
/// from the [`StatesGoalFile`].
///
/// This is distinct from [`StatesGoal`] to address the following use cases:
///
/// * Fast and offline retrieval of the goal state.
/// * Knowing what information the user used when applying a change.
///
/// [`StatesGoalFile`]: crate::paths::StatesGoalFile
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
pub type StatesGoalStored = States<GoalStored>;

impl From<States<Goal>> for States<GoalStored> {
    fn from(states_goal: States<Goal>) -> Self {
        let States(type_map, PhantomData) = states_goal;

        Self(type_map, PhantomData)
    }
}
