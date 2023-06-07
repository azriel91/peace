use serde::{Deserialize, Serialize};

/// Marker for current state.
///
/// This is used for referential param values, where an item param value is
/// dependent on the state of a predecessor's state.
///
/// A `Current<Item::State>` is set to `Some` whenever an item's current state
/// is discovered. enabling a subsequent successor's params to access that value
/// when the successor's **goal** state function is run.
///
/// Note: A successor's goal state is dependent on the predecessor's goal
/// state, which should be in sync with its current state after
/// `ApplyFns::exec` has been executed.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Current<T>(pub Option<T>);

impl<T> std::ops::Deref for Current<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Current<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
