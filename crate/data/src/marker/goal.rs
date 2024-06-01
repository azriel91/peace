use serde::{Deserialize, Serialize};

/// Marker for goal state.
///
/// This is used for referential param values, where a step param value is
/// dependent on the state of a predecessor's state.
///
/// A `Goal<Step::State>` is set to `Some` whenever a step's goal state
/// is discovered. enabling a subsequent successor's params to access that value
/// when the successor's goal state function is run.
///
/// Note: A successor's goal state is dependent on the predecessor's goal
/// state, which should be in sync with its current state after
/// `ApplyFns::exec` has been executed.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Goal<T>(pub Option<T>);

impl<T> std::ops::Deref for Goal<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Goal<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
