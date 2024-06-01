use serde::{Deserialize, Serialize};

/// Marker for clean state.
///
/// This is used for referential param values, where a step param value is
/// dependent on the state of a predecessor's state.
///
/// A `Clean<Step::State>` is set to `Some` whenever a step's clean state is
/// needed, e.g. preparing for applying the clean state. enabling a subsequent
/// successor's params to access that value when the successor's `state_clean`
/// function is run.
///
/// Note: A successor's clean state may be dependent on its predecessor's
/// current state for state discovery.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Clean<T>(pub Option<T>);

impl<T> std::ops::Deref for Clean<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Clean<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
