use serde::{Deserialize, Serialize};

/// Marker for desired state.
///
/// This is used for referential param values, where an item spec param value is
/// dependent on the state of a predecessor's state.
///
/// A `Desired<ItemSpec::State>` is set to `Some` whenever an item spec's
/// desired state is discovered. enabling a subsequent successor's params to
/// access that value when the successor's desired state function is run.
///
/// Note: A successor's desired state is dependent on the predecessor's desired
/// state, which should be in sync with its current state after
/// `EnsureOpSpec::exec` has been executed.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Desired<T>(pub Option<T>);

impl<T> std::ops::Deref for Desired<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for Desired<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
