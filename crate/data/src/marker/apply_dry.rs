use serde::{Deserialize, Serialize};

/// Marker for apply-dry state.
///
/// This is used for referential param values, where an item spec param value is
/// dependent on the state of a predecessor's state.
///
/// An `ApplyDry<ItemSpec::State>` is set to `Some` whenever an item spec is dry
/// applied, enabling a subsequent successor's params to access that value when
/// the successor's `apply_dry` function is run.
///
/// Note: A successor's dry-applied state is dependent on the predecessor's
/// dry-applied state, which should be in sync with its saved state after
/// `ApplyOpSpec::exec_dry` has been executed.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ApplyDry<T>(pub Option<T>);

impl<T> std::ops::Deref for ApplyDry<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for ApplyDry<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
