use std::marker::PhantomData;

use crate::states::{
    ts::{Goal, GoalStored},
    States,
};

/// Goal `State`s for all `Item`s.
///
/// These are the states that each item would be in, if `Item::apply` were to be
/// run with `state_goal` as the target state.
///
/// # Implementors
///
/// If a `Step`'s goal state discovery depends on the goal `State` of
/// a previous `Item`, then you should insert the predecessor's goal
/// state into [`Resources`], and reference that in the subsequent
/// `TryFnSpec`'s [`Data`]:
///
/// ```rust
/// # use std::path::PathBuf;
/// #
/// # use peace_data::{Data, R};
/// #
/// /// Predecessor `TryFnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppUploadParams<'exec> {
///     /// Path to the application directory.
///     app_dir: W<'exec, PathBuf>,
/// }
///
/// /// Successor `TryFnSpec::Data`.
/// #[derive(Data, Debug)]
/// pub struct AppInstallParams<'exec> {
///     /// Path to the application directory.
///     app_dir: R<'exec, PathBuf>,
///     /// Configuration to use.
///     config: W<'exec, String>,
/// }
/// ```
///
/// You may reference [`StatesGoal`] in `ApplyFns::Data` for reading. It
/// is not mutable as `StatesGoal` must remain unchanged so that all
/// `Item`s operate over consistent data.
///
/// [`Data`]: peace_data::Data
/// [`Resources`]: crate::Resources
pub type StatesGoal = States<Goal>;

impl From<States<GoalStored>> for States<Goal> {
    fn from(states_goal_stored: States<GoalStored>) -> Self {
        let States(type_map, PhantomData) = states_goal_stored;

        Self(type_map, PhantomData)
    }
}
